//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta381 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1418;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta381(t15827: f64, t4837: f64, t1659: f64, t3105: f64, t1062: f64, t4797: f64, t1660: f64, t3201: f64, t1058: f64, t4798: f64, t15127: f64, t15125: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t15829, t15830, t15850, t15862, t15865, t15874, t15875) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1418(t15827, t4837, t1659, t3105, t1062, t4797, t1660, t3201, t1058, t4798, t15127, t15125);
    (t15829, t15830, t15850, t15862, t15865, t15874, t15875)
}
