//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 751/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk751(t1634: f64, t1734: f64, t179: f64, t5135: f64, t66: f64, t168: f64, t167: f64, t180: f64, t1706: f64, t1733: f64, t2592: f64, t2645: f64, t5222: f64, t5225: f64, t5227: f64, t5232: f64, t5236: f64, t5241: f64, t5244: f64, t5247: f64, t5252: f64, t5258: f64, t5261: f64, t5265: f64, t5267: f64, t5270: f64, t5275: f64, t5279: f64, t580: f64) -> (f64, f64, f64, f64) {
    let t5281 = t179 * t1734 * t1634;
    let t5285 = 1.0_f64 / t66 / t5135;
    let t5286 = t168 * t5285;
    let t5289 = 0.37792653007779990369e-1_f64 * t167 * t5286 * t180;
    let t5290 = -7.0_f64 / 16.0_f64 * t5222 - t5225 * t5227 / 4.0_f64 + 0.25724410870841842183e-2_f64 * t1733 * t5232 + 0.25724410870841842183e-2_f64 * t1733 * t5236 - 0.64311027177104605458e-3_f64 * t2645 * t5241 - 0.51448821741683684367e-2_f64 * t5244 * t5247 + 0.12862205435420921092e-2_f64 * t2592 * t5252 - 0.24009450146119052704e-1_f64 * t5258 + 3.0_f64 / 16.0_f64 * t1706 * t5261 - 35.0_f64 / 72.0_f64 * t5265 + 7.0_f64 / 48.0_f64 * t5267 - t580 * t5270 / 48.0_f64 + 0.25724410870841842183e-2_f64 * t1733 * t5275 - 0.12862205435420921092e-1_f64 * t5279 * t5281 - t5289;
    (t5281, t5286, t5289, t5290)
}
