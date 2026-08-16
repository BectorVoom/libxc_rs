//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta634 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2088;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta634(t15749: f64, t7117: f64, t25490: f64, t4845: f64, t15666: f64, t27479: f64, t3215: f64, t25577: f64, t4817: f64, t15711: f64, t7132: f64, t15655: f64, t1972: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t100329, t100332, t100334, t100336, t100342, t100343, t100345) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2088(t15749, t7117, t25490, t4845, t15666, t27479, t3215, t25577, t4817, t15711, t7132, t15655, t1972);
    (t100329, t100332, t100334, t100336, t100342, t100343, t100345)
}
