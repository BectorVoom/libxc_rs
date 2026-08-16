//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2048/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2048(t2229: f64, t59: f64, t60: f64, t535: f64, t9538: f64, t241: f64, t6597: f64, t248: f64, t555: f64, t557: f64, t12248: f64, t1372: f64) -> (f64, f64, f64, f64, f64) {
    let t40419 = t59 / t60 / t2229;
    let t40422 = 0.26851851851851851851e-2_f64 * t40419 * t535 * t9538;
    let t40445 = t6597 * t241;
    let t40449 = 13685.0_f64 / 31104.0_f64 * t555 * t40445 * t557 * t248;
    let t40492 = t12248 * t1372;
    (t40419, t40422, t40445, t40449, t40492)
}
