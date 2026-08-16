//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1954/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1954(t671: f64, t7039: f64, t2035: f64, t2363: f64, t2319: f64, t7786: f64, t2032: f64, t24001: f64, t26076: f64, t7026: f64, t7035: f64, t7435: f64, t84174: f64, t84196: f64, t84198: f64, t84200: f64, t84203: f64, t84205: f64, t84207: f64, t84220: f64, t90160: f64, t90297: f64) -> (f64, f64, f64, f64) {
    let t91854 = t7039 * t671;
    let t91857 = t2035 * t2363;
    let t91870 = t7786 * t2319;
    let t91888 = -160.0_f64 / 9.0_f64 * t84174 + 80.0_f64 / 9.0_f64 * t84196 + 80.0_f64 / 9.0_f64 * t84198 + 40.0_f64 / 9.0_f64 * t84200 + 32.0_f64 / 9.0_f64 * t84203 + 16.0_f64 / 9.0_f64 * t84205 + 32.0_f64 / 9.0_f64 * t84207 - 80.0_f64 / 3.0_f64 * t84220 - 2.0_f64 / 3.0_f64 * t90160 * t2032 - 4.0_f64 / 3.0_f64 * t26076 * t7035 - 2.0_f64 / 3.0_f64 * t7435 * t24001 - 5.0_f64 / 3.0_f64 * t7026 * t90297;
    (t91854, t91857, t91870, t91888)
}
