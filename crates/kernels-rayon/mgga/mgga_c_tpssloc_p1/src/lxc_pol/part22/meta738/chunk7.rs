//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2429/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2429(t10756: f64, t10825: f64, t14332: f64, t14369: f64, t1581: f64, t17350: f64, t17355: f64, t21115: f64, t21195: f64, t21198: f64, t21247: f64, t2856: f64, t41984: f64, t42149: f64, t4411: f64, t4472: f64, t48789: f64, t49096: f64, t5762: f64, t5775: f64, t5790: f64, t5791: f64, t60338: f64, t68758: f64, t68926: f64, t68995: f64, t69066: f64, t69079: f64, t69093: f64, t69105: f64, t69118: f64, t69130: f64, t69143: f64, t69156: f64, t924: f64, t932: f64, t950: f64) -> f64 {
    let t69180 = 3.0_f64 * t4411 * t17350 + 0.96491876992155210402e2_f64 * t48789 * t5762 - t68926 - 0.19298375398431042081e3_f64 * t41984 * t21115 + 1.0_f64 * t2856 * t21195 + 1.0_f64 * t924 * (t69066 + t69079 + t69093 + t69105 + t69118 + t69130 + t69143 + t69156) * t932 + 0.2069040516770936012e4_f64 * t42149 * t21198 + 0.17544670867903938621e1_f64 * t60338 * t1581 + 0.17544670867903938621e1_f64 * t17355 * t4472 + 0.17544670867903938621e1_f64 * t14332 * t5791 + 0.30762056574649219973e4_f64 * t10756 * t5790 * t14369 * t950 - 0.19751673498613801407e-1_f64 * t68758 + t68995 - 0.35089341735807877242e1_f64 * t49096 * t5775 + 0.35089341735807877242e1_f64 * t10825 * t21247;
    t69180
}
