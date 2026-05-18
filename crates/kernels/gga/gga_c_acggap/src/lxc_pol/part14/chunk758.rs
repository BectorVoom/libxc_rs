//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 758/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk758<F: Float>(t1988: F, t2294: F, t1089: F, t2288: F, t3201: F, t598: F, t7312: F, t8447: F, t8451: F, t8453: F, t8455: F, t8459: F, t8466: F, t8470: F, t8474: F, t8478: F, t8482: F, t8487: F, t8492: F) -> (F, F, F) {
    let t8494 = t1988 * t2294;
    let t8497 = t1089 * t3201 * t2288;
    let t8498 = t598 * t8497;
    let t8500 = F::new(0.15724046144802076034e-2) * t8447 + F::new(0.94344276868812456204e-3) * t8451 + F::new(0.42874018118069736972e-3) * t8453 - F::new(0.17149607247227894789e-2) * t8455 + t7312 - F::new(0.7862023072401038017e-3) * t8459 - F::new(0.47172138434406228102e-2) * t8466 + F::new(0.15724046144802076034e-2) * t8470 - F::new(0.23586069217203114051e-2) * t8474 + F::new(0.31448092289604152068e-3) * t8478 - F::new(0.10718504529517434243e-3) * t8482 + F::new(0.47172138434406228102e-3) * t8487 + F::new(0.31448092289604152068e-3) * t8492 - F::new(0.21437009059034868486e-3) * t8494 - F::new(0.21437009059034868486e-3) * t8498;
    (t8494, t8497, t8500)
}
