//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 824/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk824<F: Float>(t2042: F, t2113: F, t1936: F, t7553: F, t572: F, t2040: F, t2115: F, t573: F, t8616: F, t8725: F, t197: F, t532: F) -> (F, F, F) {
    let t8728 = F::cast_from(3.0_f64) * t2113 * t2042;
    let t8731 = t7553 * t1936;
    let t8733 = F::cast_from(6.0_f64) * t572 * t8731;
    let t8734 = F::cast_from(3.0_f64) * t2040 * t2115 + t573 * t8725 + t8616 + t8728 + t8733;
    let t8995 = t197 * t532;
    (t8731, t8734, t8995)
}
