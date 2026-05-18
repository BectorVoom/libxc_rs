//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 820/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk820<F: Float>(t256: F, t7501: F, t7342: F, t7504: F, t248: F, t2516: F, t243: F, t2520: F, t7747: F, t7514: F, t7517: F, t7520: F, t7529: F, t7538: F, t7544: F, t7553: F, t7555: F, t7558: F, t7560: F, t7563: F, t7566: F, t7571: F, t7573: F) -> (F, F, F, F, F, F) {
    let t7753 = t256 * t7501;
    let t7754 = t7342 * t7504;
    let t7758 = F::new(1.0) / t2516 / t248;
    let t7759 = t243 * t7758;
    let t7760 = t7747 * t2520;
    let t7777 = F::new(0.264729375e1) * t7514 - F::new(0.52945875e1) * t7517 + F::new(0.94674375e0) * t7520 + F::new(0.6311625e0) * t7553 + F::new(0.3529725e1) * t7555 - F::new(0.157790625e0) * t7558 - F::new(0.41678000000000000001e0) * t7560 + F::new(0.20839e0) * t7563 - F::new(0.62517e0) * t7566 - F::new(0.103295e1) * t7529 + F::new(0.20659e1) * t7538 - F::new(0.309885e1) * t7544 - F::new(0.34731666666666666667e0) * t7571 + F::new(0.20839e0) * t7573;
    (t7753, t7754, t7758, t7759, t7760, t7777)
}
