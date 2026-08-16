//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2194/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2194(t16841: f64, t46741: f64, t17017: f64, t9638: f64, t41107: f64, t5593: f64, t16914: f64, t17009: f64, t41115: f64, t13258: f64, t16932: f64, t16937: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t58353 = t46741 * t16841;
    let t58363 = t9638 * t17017;
    let t58373 = t41107 * t5593;
    let t58379 = t9638 * t16914;
    let t58381 = t9638 * t17009;
    let t58421 = t41115 * t5593;
    let t58425 = t13258 * t16932;
    let t58427 = t13258 * t16937;
    (t58353, t58363, t58373, t58379, t58381, t58421, t58425, t58427)
}
