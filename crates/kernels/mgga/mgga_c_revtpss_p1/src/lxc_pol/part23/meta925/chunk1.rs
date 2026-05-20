//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2997/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2997<F: Float>(t1045: F, t11774: F, t11866: F, t15700: F, t15701: F, t15926: F, t16089: F, t16226: F, t20038: F, t20040: F, t20105: F, t23964: F, t23980: F, t23994: F, t247: F, t3092: F, t3106: F, t3115: F, t3116: F, t3117: F, t3155: F, t3162: F, t42328: F, t4579: F, t4837: F, t4900: F, t53676: F, t54079: F, t54818: F, t55141: F, t6267: F, t66187: F, t66328: F, t66332: F, t78812: F, t79463: F, t79467: F, t79474: F, t79480: F, t79500: F, t79505: F, t906: F) -> F {
    let t79514 = F::cast_from(0.42874018118069736972e-3_f64) * t42328 * t66187 * t3162 * t20038 - F::cast_from(0.17149607247227894789e-2_f64) * t15700 * t15701 * t79463 - F::cast_from(0.17149607247227894789e-2_f64) * t16226 * t15701 * t79467 - F::cast_from(0.85748036236139473944e-3_f64) * t55141 * t20040 - F::cast_from(0.57165357490759649296e-3_f64) * t79474 - F::cast_from(0.17149607247227894789e-2_f64) * t16226 * t66187 * t3155 * t4579 + F::cast_from(0.42874018118069736972e-3_f64) * t4837 * t247 * t3116 * t79480 - F::cast_from(0.57165357490759649295e-3_f64) * t66328 - F::cast_from(0.85748036236139473944e-3_f64) * t11774 * t54818 * t6267 - F::cast_from(0.33875767401931644028e-2_f64) * t3106 * t23980 + t54079 - F::cast_from(0.85748036236139473944e-3_f64) * t66332 - F::cast_from(0.21437009059034868486e-3_f64) * t53676 * t3117 * t78812 * t4900 - F::cast_from(0.64311027177104605458e-3_f64) * t15926 * t20105 - F::cast_from(0.64311027177104605458e-3_f64) * t11866 * t23994 - F::cast_from(0.64311027177104605458e-3_f64) * t3115 * t3117 * t79500 * t1045 - F::cast_from(0.64311027177104605458e-3_f64) * t3115 * t3117 * t79505 * t1045 + F::cast_from(0.85748036236139473947e-3_f64) * t16089 * t3092 * t23964 * t906;
    t79514
}
