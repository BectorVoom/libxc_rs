//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2089/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2089(t212: f64, t2553: f64, t2586: f64, t9523: f64, t9525: f64, t9577: f64, t116: f64, t244: f64, t2379: f64, t2563: f64, t9529: f64, t207: f64, t40419: f64, t9538: f64) -> (f64, f64, f64, f64, f64) {
    let t41142 = t2586 * t9523 * t212 * t2553;
    let t41144 = t9577 * t9525;
    let t41146 = t244 * t116;
    let t41149 = t2586 * t41146 * t212 * t2379;
    let t41151 = t2563 * t9529;
    let t41155 = 0.26851851851851851851e-2_f64 * t40419 * t207 * t9538;
    (t41142, t41144, t41149, t41151, t41155)
}
