//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2751/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2751(t40722: f64, t12939: f64, t16619: f64, t2244: f64, t46234: f64, t46236: f64, t40729: f64, t40733: f64, t2517: f64, t5398: f64, t707: f64, t10130: f64, t12935: f64, t193: f64, t39472: f64, t39476: f64, t40721: f64, t40732: f64, t5527: f64, t5544: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t57983 = 0.11393789434848516922e-2_f64 * t40722;
    let t57986 = 24.0_f64 * t12939 * t16619 * t2244;
    let t57987 = 0.69263436422725855034e2_f64 * t46234;
    let t57988 = 0.46785788981077169656e1_f64 * t46236;
    let t57989 = 12.0_f64 * t40729;
    let t57990 = 0.70178683471615754484e1_f64 * t40733;
    let t57992 = t707 * t2517 * t5398;
    let t57993 = 4.0_f64 * t57992;
    let t57994 = 6.0_f64 * t10130 * t193 * t5527 + 6.0_f64 * t12935 * t193 * t5544 - t39472 - t39476 - t40721 - t40732 - t57983 + t57986 - t57987 + t57988 + t57989 - t57990 + t57993;
    (t57983, t57986, t57987, t57988, t57989, t57990, t57993, t57994)
}
