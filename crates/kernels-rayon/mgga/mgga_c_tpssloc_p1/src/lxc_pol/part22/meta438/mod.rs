//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta438 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1779;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1780;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1781;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta438(t19591: f64, t592: f64, t6328: f64, t11984: f64, t15880: f64, t15889: f64, t15894: f64, t19543: f64, t19574: f64, t19576: f64, t19577: f64, t19581: f64, t19588: f64, t19589: f64, t19590: f64, t3918: f64, t3919: f64, t5122: f64, t5126: f64, t5161: f64, t5187: f64, t5308: f64, t6347: f64, t9457: f64, t9476: f64, t9484: f64, t25: f64, t3701: f64, t6463: f64, t15909: f64, t5127: f64, t11987: f64, t6305: f64, t3704: f64, t5397: f64, t1298: f64, t16557: f64, t2219: f64, t5170: f64, t606: f64, zeta_threshold: f64, t28: f64, t12000: f64, t6312: f64, t3711: f64, t5966: f64, t1081: f64, t1302: f64, t18196: f64, t5178: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t19592, t19593, t19594, t19595) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1779(t19591, t592, t6328, t11984, t15880, t15889, t15894, t19543, t19574, t19576, t19577, t19581, t19588, t19589, t19590, t3918, t3919, t5122, t5126, t5161, t5187, t5308, t6347, t9457, t9476, t9484);
        let (t19596, t19599, t19603, t19606, t19617) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1780(t25, t3701, t6463, t15909, t5127, t5187, t11987, t6305, t3704, t5397, t1298, t16557, t2219, t5170, t606, zeta_threshold);
        let (t19618, t19631) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1781(t28, t12000, t6312, t3711, t5966, t1081, t1302, t18196, t2219, t5178, t19617, zeta_threshold);
    (t19592, t19593, t19594, t19595, t19596, t19599, t19603, t19606, t19618, t19631)
}
