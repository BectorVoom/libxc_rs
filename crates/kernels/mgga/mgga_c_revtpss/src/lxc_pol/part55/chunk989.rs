//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 989/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk989<F: Float>(t117: F, t34775: F, t1518: F, t2055: F, t29427: F, t33287: F, t33644: F, t33646: F, t34308: F, t34310: F, t34312: F, t34320: F, t34323: F, t34325: F, t34446: F, t7586: F, t7983: F, t8564: F) -> (F, F) {
    let t34776 = t34775 * t117;
    let t34788 = 2.0 * t1518 * t33287 + 2.0 * t2055 * t29427 + 2.0 * t2055 * t34446 + 2.0 * t7586 * t7983 + t33644 + t33646 + t34308 + t34310 + t34312 + t34320 + t34323 + t34325 + t34776 + t8564;
    (t34776, t34788)
}
