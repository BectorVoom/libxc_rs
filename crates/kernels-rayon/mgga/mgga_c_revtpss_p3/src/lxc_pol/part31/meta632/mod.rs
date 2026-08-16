//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta632 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2086;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta632(t4797: f64, t7131: f64, t15682: f64, t25517: f64, t4857: f64, t16163: f64, t7122: f64, t15772: f64, t7132: f64, t15984: f64, t1058: f64, t27464: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t100230, t100240, t100255, t100261, t100262, t100268, t100270) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2086(t4797, t7131, t15682, t25517, t4857, t16163, t7122, t15772, t7132, t15984, t1058, t27464);
    (t100230, t100240, t100255, t100261, t100262, t100268, t100270)
}
