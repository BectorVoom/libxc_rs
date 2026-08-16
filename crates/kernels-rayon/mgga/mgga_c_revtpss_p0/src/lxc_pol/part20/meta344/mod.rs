//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta344 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1271;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta344(t16558: f64, t342: f64, t12050: f64, t3154: f64, t3151: f64, t12046: f64, t378: f64, t357: f64, t379: f64, t994: f64, t1214: f64, t5333: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t16559, t16561, t16565, t16566, t16568, t16603, t16696) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1271(t16558, t342, t12050, t3154, t3151, t12046, t378, t357, t379, t994, t1214, t5333);
    (t16559, t16561, t16565, t16566, t16568, t16603, t16696)
}
