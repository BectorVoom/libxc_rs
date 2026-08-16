//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta426 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1648;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1649;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1650;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta426(t25: f64, t3701: f64, t6463: f64, t15909: f64, t5127: f64, t5187: f64, t11987: f64, t6305: f64, t3704: f64, t5397: f64, t1298: f64, t16557: f64, t2219: f64, t5170: f64, t606: f64, zeta_threshold: f64, t28: f64, t12000: f64, t6312: f64, t3711: f64, t5966: f64, t1081: f64, t1302: f64, t18196: f64, t5178: f64, t1834: f64, t5210: f64, t1807: f64, t5318: f64, t1842: f64, t5353: f64, t3887: f64, t1814: f64, t5333: f64, t1338: f64, t6434: f64, t1352: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t19596, t19599, t19603, t19617) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1648(t25, t3701, t6463, t15909, t5127, t5187, t11987, t6305, t3704, t5397, t1298, t16557, t2219, t5170, t606, zeta_threshold);
        let t19631 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1649(t28, t12000, t6312, t3711, t5966, t1081, t1302, t18196, t2219, t5178, t19617, zeta_threshold);
        let (t19635, t19644, t19647, t19648, t19654, t19658) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1650(t1834, t5210, t1807, t5318, t1842, t5353, t3887, t1814, t5333, t1338, t6434, t1352);
    (t19596, t19599, t19603, t19631, t19635, t19644, t19647, t19648, t19654, t19658)
}
