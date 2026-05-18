//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1077/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1077<F: Float>(t2055: F, t27060: F, t29432: F, t32176: F, t32178: F, t32642: F, t32644: F, t32646: F, t32654: F, t32657: F, t32659: F, t33286: F, t33287: F, t670: F, t7373: F, t7586: F, t8564: F) -> F {
    let t33296 = F::new(2.0) * t2055 * t27060 + F::new(2.0) * t2055 * t29432 + F::new(2.0) * t33287 * t670 + F::new(2.0) * t7373 * t7586 + t32176 + t32178 + t32642 + t32644 + t32646 + t32654 + t32657 + t32659 + t33286 + t8564;
    t33296
}
