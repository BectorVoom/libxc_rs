//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta633 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2087;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta633(t3201: f64, t7801: f64, t1058: f64, t27467: f64, t15775: f64, t7132: f64, t100054: f64, t3299: f64, t4857: f64, t7125: f64, t25495: f64, t4845: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t100272, t100275, t100289, t100302, t100324, t100327) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2087(t3201, t7801, t1058, t27467, t15775, t7132, t100054, t3299, t4857, t7125, t25495, t4845);
    (t100272, t100275, t100289, t100302, t100324, t100327)
}
