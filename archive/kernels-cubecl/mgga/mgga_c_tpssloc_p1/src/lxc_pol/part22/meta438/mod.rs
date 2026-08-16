//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta438 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1779;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1780;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1781;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta438<F: Float>(t19591: F, t592: F, t6328: F, t11984: F, t15880: F, t15889: F, t15894: F, t19543: F, t19574: F, t19576: F, t19577: F, t19581: F, t19588: F, t19589: F, t19590: F, t3918: F, t3919: F, t5122: F, t5126: F, t5161: F, t5187: F, t5308: F, t6347: F, t9457: F, t9476: F, t9484: F, t25: F, t3701: F, t6463: F, t15909: F, t5127: F, t11987: F, t6305: F, t3704: F, t5397: F, t1298: F, t16557: F, t2219: F, t5170: F, t606: F, zeta_threshold: F, t28: F, t12000: F, t6312: F, t3711: F, t5966: F, t1081: F, t1302: F, t18196: F, t5178: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t19592, t19593, t19594, t19595) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1779::<F>(t19591, t592, t6328, t11984, t15880, t15889, t15894, t19543, t19574, t19576, t19577, t19581, t19588, t19589, t19590, t3918, t3919, t5122, t5126, t5161, t5187, t5308, t6347, t9457, t9476, t9484);
        let (t19596, t19599, t19603, t19606, t19617) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1780::<F>(t25, t3701, t6463, t15909, t5127, t5187, t11987, t6305, t3704, t5397, t1298, t16557, t2219, t5170, t606, zeta_threshold);
        let (t19618, t19631) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1781::<F>(t28, t12000, t6312, t3711, t5966, t1081, t1302, t18196, t2219, t5178, t19617, zeta_threshold);
    (t19592, t19593, t19594, t19595, t19596, t19599, t19603, t19606, t19618, t19631)
}
