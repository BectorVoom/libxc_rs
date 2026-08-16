//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1699/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1699(t88980: f64, t88981: f64, t88983: f64, t88996: f64, t1041: f64, t1042: f64, t1045: f64, t1592: f64, t16067: f64, t16089: f64, t16199: f64, t19450: f64, t19968: f64, t23830: f64, t23839: f64, t23929: f64, t23934: f64, t23964: f64, t3092: f64, t3117: f64, t3127: f64, t357: f64, t373: f64, t42121: f64, t43291: f64, t4801: f64, t4892: f64, t4899: f64, t54500: f64, t54564: f64, t6299: f64, t6308: f64, t6331: f64, t65339: f64, t78873: f64, t79071: f64, t88901: f64, t88925: f64, t88948: f64) -> (f64, f64) {
    let t88998 = t88980 + t88981 + t88983 + t88996;
    let t89009 = -0.17149607247227894789e-2_f64 * t19968 * t6331 - 0.51448821741683684368e-2_f64 * t43291 * t3117 * t88948 * t1045 + 0.34299214494455789578e-2_f64 * t16089 * t3092 * t23964 * t1592 + 0.12862205435420921092e-2_f64 * t16067 * t3117 * t19450 * t357 * t6299 + 0.17149607247227894789e-2_f64 * t4892 * t3117 * t78873 * t23929 - 0.85748036236139473944e-3_f64 * t4899 * t3117 * t78873 * t23934 + 0.51448821741683684368e-2_f64 * t54500 * t23839 + 0.17149607247227894789e-2_f64 * t3127 * t1042 * t4801 * t88901 + 0.57165357490759649296e-2_f64 * t3127 * t1042 * t16199 * t88925 + 0.21437009059034868486e-3_f64 * t1041 * t1042 * t373 * t88998 * t1045 + 0.25724410870841842184e-2_f64 * t65339 * t6308 + 0.51448821741683684368e-2_f64 * t54564 * t23830 - t42121 + 0.17149607247227894789e-2_f64 * t79071;
    (t88998, t89009)
}
