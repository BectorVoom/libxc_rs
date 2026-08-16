//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 716/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk716(t1701: f64, t4857: f64, t10926: f64, t1725: f64, t1726: f64, t4903: f64, t1724: f64, t4911: f64, t10903: f64, t1746: f64, t4954: f64, t10978: f64, t10983: f64, t10984: f64, t11067: f64, t11102: f64, t11116: f64, t1706: f64, t1735: f64, t4853: f64, t4858: f64, t4860: f64, t4904: f64, t4909: f64, t4912: f64, t621: f64) -> f64 {
    let t11119 = t1701 * t4857;
    let t11122 = t10926 * t1725;
    let t11125 = t1726 * t4903;
    let t11129 = t4903 * t4911 * t1724;
    let t11133 = t4954 * t10903 * t1746;
    let t11136 = 3.0_f64 * t4853 * t4904 + 0.48245472966453314466e2_f64 * t10978 * t4912 - 0.96490945932906628932e2_f64 * t10983 * t10984 + 1.0_f64 * t1706 * t11067 - 0.58482233974552040708e0_f64 * t1735 * t11102 - 0.62182e-1_f64 * t11116 * t621 - 6.0_f64 * t11119 * t4860 + 6.0_f64 * t4909 * t11122 - 6.0_f64 * t4858 * t11125 + 0.48245472966453314466e2_f64 * t4909 * t11129 - 0.35089340384731224426e1_f64 * t1735 * t11133;
    t11136
}
