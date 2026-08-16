//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1123/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1123(t2512: f64, t39378: f64, t9489: f64, t1294: f64, t2509: f64, t39389: f64, t763: f64, t9697: f64, t3684: f64, t2371: f64, t2393: f64, t2528: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t39488 = t9489 * t39378 * t2512;
    let t39490 = 0.6233709278045326953e3_f64 * t1294 * t39488;
    let t39494 = t2509 * t39389 * t2512;
    let t39496 = 0.51947577317044391277e2_f64 * t1294 * t39494;
    let t39497 = t9697 * t763;
    let t39499 = 0.67471172535210825684e-1_f64 * t3684 * t39497;
    let t39500 = t2393 * t2371;
    let t39502 = 0.86748650402413918736e-1_f64 * t3684 * t39500;
    let t39503 = t2393 * t2528;
    (t39488, t39490, t39494, t39496, t39497, t39499, t39500, t39502, t39503)
}
