//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 583/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk583(t1737: f64, t1746: f64, t4948: f64, t1736: f64, t4929: f64, t633: f64, t1706: f64, t1726: f64, t1735: f64, t1747: f64, t45: f64, t4850: f64, t4853: f64, t4858: f64, t4860: f64, t4904: f64, t4909: f64, t4912: f64, t4920: f64, t4924: f64, t4931: f64, t621: f64, t634: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4950 = t1737 * t4948 * t1746;
    let t4953 = t1736 * t1736;
    let t4954 = 1.0_f64 / t4953;
    let t4955 = t4954 * t4929;
    let t4956 = t633 * t633;
    let t4957 = 1.0_f64 / t4956;
    let t4958 = t4955 * t4957;
    let t4961 = -0.62182e-1_f64 * t4850 * t621 + 2.0_f64 * t4853 * t1726 - 2.0_f64 * t4858 * t4860 + 1.0_f64 * t1706 * t4904 + 0.16081824322151104822e2_f64 * t4909 * t4912 + 0.19751789702565206229e-1_f64 * t45 * t4920 * t634 - 0.11696446794910408142e1_f64 * t4924 * t1747 + 0.11696446794910408142e1_f64 * t1735 * t4931 - 0.58482233974552040708e0_f64 * t1735 * t4950 - 0.17315755899375863299e2_f64 * t1735 * t4958;
    (t4950, t4953, t4954, t4956, t4957, t4958, t4961)
}
