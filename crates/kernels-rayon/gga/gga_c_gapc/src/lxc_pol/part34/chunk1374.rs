//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1374/1427 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1374(t33482: f64, t33464: f64, t33474: f64, t36596: f64, t36597: f64, t36599: f64, t36600: f64, t36601: f64, t36602: f64, t36604: f64, t36605: f64, t33487: f64) -> (f64, f64) {
    let t36606 = 0.77294542590142724634e-6_f64 * t33482;
    let t36607 = -t36596 - t36597 - 0.18115908419564701085e-6_f64 * t33464 + t36599 - t36600 + t36601 + t36602 - 0.5691280480400994668e-7_f64 * t33474 - t36604 + t36605 + t36606;
    let t36609 = 0.1374296967252737644e-5_f64 * t33487;
    (t36607, t36609)
}
