//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1212/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1212(t1915: f64, t2793: f64, t1088: f64, t5829: f64, t1107: f64, t1977: f64, t1096: f64, t1108: f64, t17391: f64, t1916: f64, t1917: f64, t1918: f64, t1932: f64, t21062: f64, t21082: f64, t2816: f64, t5484: f64, t5830: f64, t5832: f64, t5838: f64, t5865: f64, t5887: f64, t5890: f64, t5894: f64, t5897: f64, t5906: f64, t695: f64, t702: f64, t703: f64, t7231: f64, t7234: f64, t7237: f64, t7240: f64, t7315: f64, t7324: f64, t7408: f64) -> f64 {
    let t21087 = t2793 * t1915;
    let t21090 = t1088 * t5829;
    let t21093 = t1977 * t1107;
    let t21121 = 0.35089341735807877242e1_f64 * t7315 * t5906 + 1.0_f64 * t695 * (t21062 + t21082) * t703 - 6.0_f64 * t21087 * t1918 - 0.19298375398431042081e3_f64 * t21090 * t5832 + 0.10526802520742363173e2_f64 * t21093 * t5890 + 0.51947577317044391277e2_f64 * t7315 * t5894 + 0.96491876992155210402e2_f64 * t7324 * t5887 - 0.14035736694323150897e2_f64 * t5838 * t1108 * t5484 - 12.0_f64 * t5897 * t7231 - 6.0_f64 * t1916 * t7408 * t702 - 6.0_f64 * t1916 * t2816 * t1932 - 0.57895126195293126242e3_f64 * t5830 * t7240 * t1917 - 6.0_f64 * t5897 * t7234 - 2.0_f64 * t1916 * t1096 * t5865 - 0.57895126195293126242e3_f64 * t17391 * t7237;
    t21121
}
