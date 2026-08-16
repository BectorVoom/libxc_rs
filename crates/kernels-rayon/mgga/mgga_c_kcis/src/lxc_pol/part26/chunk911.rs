//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 911/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk911(t17771: f64, t5618: f64, t3944: f64, t7019: f64, t5619: f64, t1578: f64, t7024: f64, t45: f64, t6996: f64, t1345: f64, t1357: f64, t21172: f64, t21174: f64, t21176: f64, t21178: f64, t21180: f64, t21293: f64, t21295: f64, t21311: f64, t21316: f64, t21320: f64, t21324: f64, t21327: f64, t5590: f64, t5615: f64) -> (f64, f64, f64, f64) {
    let t21330 = t5618 * t17771;
    let t21333 = t3944 * t7019;
    let t21334 = t21333 * t5619;
    let t21337 = t7024 * t1578;
    let t21342 = t45 * t6996;
    let t21345 = t21172 + t21174 + t21176 - t21178 + t21180 + t21293 + t21295 + 0.19751789702565206229e-1_f64 * t45 * t21311 - 0.1025389702100779493e4_f64 * t1345 * t21316 + 0.1038945353962551798e3_f64 * t1345 * t21320 + 0.11696446794910408142e1_f64 * t1345 * t21324 + 0.23392893589820816284e1_f64 * t1345 * t21327 - 0.34631511798751726598e2_f64 * t1345 * t21330 - 0.17315755899375863299e2_f64 * t1345 * t21334 - 0.35089340384731224426e1_f64 * t1345 * t21337 - 0.11696446794910408142e1_f64 * t5590 * t5615 - 0.58482233974552040708e0_f64 * t21342 * t1357;
    (t21330, t21334, t21337, t21345)
}
