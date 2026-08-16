//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta763 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2559;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta763(t16543: f64, t3046: f64, t4746: f64, t4995: f64, t15669: f64, t3286: f64, t1651: f64, t378: f64, t342: f64, t43400: f64, t3057: f64, t12077: f64, t1647: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t55701, t55732, t55747, t55764, t55805, t55887, t55899) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2559(t16543, t3046, t4746, t4995, t15669, t3286, t1651, t378, t342, t43400, t3057, t12077, t1647);
    (t55701, t55732, t55747, t55764, t55805, t55887, t55899)
}
