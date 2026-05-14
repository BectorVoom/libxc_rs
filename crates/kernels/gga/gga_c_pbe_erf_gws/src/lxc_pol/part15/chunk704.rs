//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 704/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk704<F: Float>(t1289: F, t387: F, t13: F, t1292: F, t30: F, t4510: F, t2704: F, t2718: F, t4518: F, t4521: F, t4524: F, t4529: F, t4531: F, t4533: F, t441: F, t1257: F, t433: F) -> (F, F, F) {
    let t4658 = 1.0 / t1289 / t387;
    let t4659 = t13 * t4658;
    let t4661 = 1.0 / t1292 / t30;
    let t4662 = t4510 * t4661;
    let t4663 = t4659 * t4662;
    let t4664 = 0.51725014705706168417e3 * t4663;
    let t4673 = -0.47063e1 * t4518 + 0.31375333333333333334e1 * t4521 - 0.36604555555555555556e1 * t4524 - 0.16068111111111111111e1 * t2704 + 0.28051666666666666666e0 * t4529 - 0.56103333333333333332e0 * t4531 - 0.6545388888888888889e0 * t4533 - 0.46308888888888888888e0 * t2718;
    let t4674 = t4673 * t441;
    let t4678 = 1.0 / t1257 / t433;
    (t4664, t4674, t4678)
}
