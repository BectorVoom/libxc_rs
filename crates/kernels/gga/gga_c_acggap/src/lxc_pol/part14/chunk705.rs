//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 705/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk705<F: Float>(t598: F, t8497: F, t7312: F, t8447: F, t8451: F, t8453: F, t8455: F, t8459: F, t8466: F, t8470: F, t8474: F, t8478: F, t8482: F, t8487: F, t8492: F, t8494: F) -> (F,) {
    let t8498 = t598 * t8497;
    let t8500 = 0.15724046144802076034e-2 * t8447 + 0.94344276868812456204e-3 * t8451 + 0.42874018118069736972e-3 * t8453 - 0.17149607247227894789e-2 * t8455 + t7312 - 0.7862023072401038017e-3 * t8459 - 0.47172138434406228102e-2 * t8466 + 0.15724046144802076034e-2 * t8470 - 0.23586069217203114051e-2 * t8474 + 0.31448092289604152068e-3 * t8478 - 0.10718504529517434243e-3 * t8482 + 0.47172138434406228102e-3 * t8487 + 0.31448092289604152068e-3 * t8492 - 0.21437009059034868486e-3 * t8494 - 0.21437009059034868486e-3 * t8498;
    (t8500,)
}
