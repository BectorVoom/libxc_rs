//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2845/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2845<F: Float>(t51973: F, t41281: F, t41283: F, t41285: F, t41287: F, t41289: F, t41292: F, t41307: F, t51961: F, t51965: F, t51967: F, t51971: F) -> F {
    let t51974 = F::cast_from(0.40256666666666666668e0_f64) * t51973;
    let t51975 = F::cast_from(0.55190000000000000001e0_f64) * t41281 - F::new(0.11038e0) * t41283 - F::new(0.27595e0) * t41285 - F::cast_from(0.91983333333333333335e-1_f64) * t41287 + F::new(0.5519e-1) * t41289 + F::cast_from(0.24528888888888888889e-1_f64) * t41292 + t41307 + F::new(0.36231e1) * t51961 - F::cast_from(0.10064166666666666667e1_f64) * t51965 + F::cast_from(0.30192500000000000001e0_f64) * t51967 - F::new(0.301925e0) * t51971 - t51974;
    t51975
}
