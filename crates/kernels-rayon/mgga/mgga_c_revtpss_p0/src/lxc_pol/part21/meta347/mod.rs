//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta347 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1681;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1682;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta347(t3129: f64, t3172: f64, t3127: f64, t3135: f64, t1041: f64, t1065: f64, t3059: f64, t906: f64, t1042: f64, t1024: f64, t3105: f64, t3151: f64, t3153: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11643, t11644, t11648, t11649, t11652, t11653, t11656) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1681(t3129, t3172, t3127, t3135, t1041, t1065, t3059, t906, t1042, t1024, t3105);
        let t11659 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1682(t3151, t3153);
    (t11643, t11644, t11648, t11649, t11652, t11653, t11656, t11659)
}
