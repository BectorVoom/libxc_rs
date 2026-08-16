//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta332 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1249;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1250;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta332(t3145: f64, t334: f64, t368: f64, t3153: f64, t73: f64, t246: f64, t676: f64, t1046: f64, t1041: f64, t1038: f64, t3229: f64, t1036: f64, t1033: f64, t3169: f64, t3173: f64, t2866: f64, t914: f64, t2923: f64, t910: f64, t287: f64, t2922: f64, t275: f64, t11132: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11243, t11249, t11262, t11264, t11267) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1249(t3145, t334, t368, t3153, t73, t246, t676, t1046, t1041, t1038, t3229, t1036);
        let (t11268, t11271, t11289, t11294, t11299, t11304) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1250(t1033, t11267, t3169, t3173, t2866, t914, t2923, t910, t287, t2922, t275, t11132);
    (t11243, t11249, t11262, t11264, t11268, t11271, t11289, t11294, t11299, t11304)
}
