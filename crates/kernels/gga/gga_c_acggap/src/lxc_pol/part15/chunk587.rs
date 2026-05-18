//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 587/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk587<F: Float>(t50: F, t5455: F, t1369: F, t238: F, t52: F, t5460: F, t5465: F, t822: F, t5459: F, t59: F, t85: F, t4030: F, t2635: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t51 = t50 <= zeta_threshold;
    let t5468 = -t5455;
    let t5472 = piecewise3::<f64>(t51, F::new(0.0), -F::new(8.0) / F::new(27.0) * t5460 * t238 - F::new(16.0) / F::new(9.0) * t1369 * t822 + F::new(4.0) / F::new(9.0) * t5465 * t238 + F::new(4.0) / F::new(3.0) * t52 * t5468);
    let t5474 = (t5459 + t5472) * t59;
    let t5475 = t5474 * t85;
    let t5476 = F::new(0.19751673498613801407e-1) * t5475;
    let t5477 = F::new(0.48830526149350786811e-3) * t4030;
    let t5478 = F::new(12.0) * t2635;
    (t5468, t5474, t5476, t5477, t5478)
}
