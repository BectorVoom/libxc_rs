//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta373 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1703;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1704;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta373(t1660: f64, t3201: f64, t1058: f64, t4798: f64, t15127: f64, t15125: f64, t15191: f64, t4794: f64, t11243: f64, t72: f64, t3088: f64, t12078: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15862, t15865, t15874, t15875, t15876, t15892, t15904, t15905) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1703(t1660, t3201, t1058, t4798, t15127, t15125, t15191, t4794, t11243, t72, t3088);
        let t15906 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1704(t12078, t15905);
    (t15862, t15865, t15874, t15875, t15876, t15892, t15904, t15905, t15906)
}
