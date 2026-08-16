//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta932 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3161;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3162;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta932(t13014: f64, t5373: f64, t12998: f64, t1222: f64, t140: f64, t17404: f64, t12941: f64, t5293: f64, t5274: f64, t1263: f64, t16750: f64, t17547: f64, t3704: f64, t17609: f64, t12901: f64, t17525: f64, t1261: f64, t17551: f64, t3172: f64, t3625: f64, t44250: f64, t5406: f64, t12773: f64, t17448: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t57290, t57292, t57295, t57297, t57299, t57303, t57314) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3161(t13014, t5373, t12998, t1222, t140, t17404, t12941, t5293, t5274, t1263, t16750, t17547, t3704);
        let (t57316, t57318, t57321, t57331, t57333) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3162(t17609, t3704, t12901, t17525, t1261, t17551, t3172, t3625, t44250, t5406, t12773, t17448);
    (t57290, t57292, t57295, t57297, t57299, t57303, t57314, t57316, t57318, t57321, t57331, t57333)
}
