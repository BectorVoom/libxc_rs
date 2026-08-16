//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1149/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1149(t238: f64, t244: f64, t248: f64, t40445: f64, t116: f64, t207: f64, t40419: f64, t9538: f64, t154: f64, t1891: f64, t205: f64, t792: f64, t9558: f64) -> (f64, f64, f64, f64, f64) {
    let t41139 = 13685.0_f64 / 31104.0_f64 * t238 * t40445 * t244 * t248;
    let t41146 = t244 * t116;
    let t41155 = 0.26851851851851851851e-2_f64 * t40419 * t207 * t9538;
    let t41160 = t154 * t1891;
    let t41161 = t205 * t41160;
    let t41170 = t792 * t9558;
    (t41139, t41146, t41155, t41161, t41170)
}
