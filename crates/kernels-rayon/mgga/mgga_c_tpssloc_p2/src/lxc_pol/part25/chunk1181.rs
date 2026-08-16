//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1181/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1181(t23966: f64, t9231: f64, t6492: f64, t22527: f64, t23967: f64, t22531: f64, t22546: f64, t22549: f64, t23963: f64, t23970: f64, t605: f64, t83722: f64, t83745: f64, t83778: f64, t83820: f64, t84174: f64, t84180: f64, t84183: f64, t84186: f64, t84190: f64) -> f64 {
    let t84195 = t9231 * t23966;
    let t84196 = t84195 * t6492;
    let t84198 = t23967 * t22527;
    let t84200 = t23967 * t22531;
    let t84202 = -160.0_f64 / 3.0_f64 * t84174 + 20.0_f64 * t83722 * t23970 + 10.0_f64 * t83778 * t23970 + 20.0_f64 * t22549 * t84180 + 10.0_f64 * t22549 * t84183 - 2.0_f64 * t605 * t84186 * t83820 + 30.0_f64 * t84190 * t22546 + 30.0_f64 * t23963 * t83745 + 80.0_f64 / 3.0_f64 * t84196 + 80.0_f64 / 3.0_f64 * t84198 + 40.0_f64 / 3.0_f64 * t84200;
    t84202
}
