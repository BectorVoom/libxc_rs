//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 819/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk819(t12656: f64, t825: f64, t826: f64, t12651: f64, t12708: f64, t7416: f64, t2464: f64, t2465: f64, t2684: f64, t9603: f64, t2365: f64, t28302: f64, t7390: f64) -> (f64, f64, f64, f64, f64) {
    let t41425 = t825 * t826 * t12656;
    let t41428 = t825 * t826 * t12651;
    let t41430 = t7416 * t12708;
    let t41435 = t2684 * t2464 * t2465 * t9603;
    let t41445 = t7390 * t2365 * t28302;
    (t41425, t41428, t41430, t41435, t41445)
}
