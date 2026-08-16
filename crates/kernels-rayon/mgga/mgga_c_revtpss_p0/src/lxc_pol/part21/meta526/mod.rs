//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta526 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2167;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta526(t16551: f64, t342: f64, t11631: f64, t12050: f64, t3151: f64, t15907: f64, t12077: f64, t378: f64, t3154: f64, t12046: f64, t357: f64, t3133: f64, t3302: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16552, t16554, t16555, t16558, t16559, t16561, t16562, t16565, t16566, t16568, t16569, t16573) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2167(t16551, t342, t11631, t12050, t3151, t15907, t12077, t378, t3154, t12046, t357, t3133, t3302);
    (t16552, t16554, t16555, t16558, t16559, t16561, t16562, t16565, t16566, t16568, t16569, t16573)
}
