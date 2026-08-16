//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2642/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2642(t25: f64, t53796: f64, t5154: f64, t9919: f64, t39305: f64, t3665: f64, t584: f64, t2249: f64, t606: f64, t16: f64, t5173: f64, t591: f64, t11987: f64, t11988: f64, t1298: f64, t1408: f64, t15989: f64, t15992: f64, t2: f64, t3704: f64, t39861: f64, t5170: f64, t9257: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t53797 = 0.35089341735807877242e1_f64 * t53796;
    let t53798 = t5154 * t9919;
    let t53799 = 0.35089341735807877242e1_f64 * t53798;
    let t53800 = 0.31168546390226634765e3_f64 * t39305;
    let t53805 = t584 * t3665;
    let t53808 = t606 * t2249;
    let t53814 = t16 * t606;
    let t53817 = t584 * t2249;
    let t53825 = 16.0_f64 * t5173 * t591;
    let t53827 = piecewise3(t26, 0.0_f64, -56.0_f64 / 81.0_f64 * t39861 * t1408 * t11988 + 16.0_f64 / 9.0_f64 * t11987 * t2 * t53805 + 8.0_f64 / 9.0_f64 * t15989 * t53808 - 4.0_f64 / 3.0_f64 * t3704 * t584 * t606 + 4.0_f64 * t15992 * t53814 - 4.0_f64 / 3.0_f64 * t15992 * t53817 - 2.0_f64 / 9.0_f64 * t5170 * t9257 - 8.0_f64 * t1298 * t16 + t53825);
    (t53797, t53799, t53800, t53805, t53808, t53814, t53817, t53827)
}
