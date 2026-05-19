//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1291/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1291<F: Float>(t17771: F, t5618: F, t3944: F, t7019: F, t5619: F, t1578: F, t7024: F, t45: F, t6996: F, t1345: F, t1357: F, t21172: F, t21174: F, t21176: F, t21178: F, t21180: F, t21293: F, t21295: F, t21311: F, t21316: F, t21320: F, t21324: F, t21327: F, t5590: F, t5615: F) -> (F, F, F, F) {
    let t21330 = t5618 * t17771;
    let t21333 = t3944 * t7019;
    let t21334 = t21333 * t5619;
    let t21337 = t7024 * t1578;
    let t21342 = t45 * t6996;
    let t21345 = t21172 + t21174 + t21176 - t21178 + t21180 + t21293 + t21295 + F::cast_from(0.19751789702565206229e-1_f64) * t45 * t21311 - F::cast_from(0.1025389702100779493e4_f64) * t1345 * t21316 + F::cast_from(0.1038945353962551798e3_f64) * t1345 * t21320 + F::cast_from(0.11696446794910408142e1_f64) * t1345 * t21324 + F::cast_from(0.23392893589820816284e1_f64) * t1345 * t21327 - F::cast_from(0.34631511798751726598e2_f64) * t1345 * t21330 - F::cast_from(0.17315755899375863299e2_f64) * t1345 * t21334 - F::cast_from(0.35089340384731224426e1_f64) * t1345 * t21337 - F::cast_from(0.11696446794910408142e1_f64) * t5590 * t5615 - F::cast_from(0.58482233974552040708e0_f64) * t21342 * t1357;
    (t21330, t21334, t21337, t21345)
}
