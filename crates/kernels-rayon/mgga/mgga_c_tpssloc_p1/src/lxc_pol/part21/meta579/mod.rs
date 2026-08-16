//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta579 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2303;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2304;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta579(t25: f64, t3701: f64, t6463: f64, t15909: f64, t5127: f64, t5187: f64, t11987: f64, t6305: f64, t3704: f64, t5397: f64, t1298: f64, t16557: f64, t2219: f64, t5170: f64, t606: f64, zeta_threshold: f64, t28: f64, t12000: f64, t6312: f64, t3711: f64, t5966: f64, t1081: f64, t1302: f64, t18196: f64, t5178: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t19596, t19599, t19603, t19606, t19611, t19617) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2303(t25, t3701, t6463, t15909, t5127, t5187, t11987, t6305, t3704, t5397, t1298, t16557, t2219, t5170, t606, zeta_threshold);
        let (t19618, t19623, t19631) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2304(t28, t12000, t6312, t3711, t5966, t1081, t1302, t18196, t2219, t5178, t19617, zeta_threshold);
    (t19596, t19599, t19603, t19606, t19611, t19618, t19623, t19631)
}
