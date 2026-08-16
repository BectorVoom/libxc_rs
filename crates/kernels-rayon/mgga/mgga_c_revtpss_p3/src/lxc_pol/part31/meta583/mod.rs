//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta583 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2003;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2004;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta583(t25516: f64, t3278: f64, t11223: f64, t1976: f64, t27639: f64, t995: f64, t19482: f64, t988: f64, t25610: f64, t25604: f64, t7156: f64, t3268: f64, t7143: f64, t3057: f64, t25460: f64, t25698: f64, t1071: f64, t7150: f64, t8521: f64, t359: f64, t42066: f64, t3143: f64, t36870: f64, t1983: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t93821, t93884, t93890, t93893, t93897, t93904, t93920) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2003(t25516, t3278, t11223, t1976, t27639, t995, t19482, t988, t25610, t25604, t7156, t3268, t7143);
        let (t93921, t93928, t93963, t93968, t93983) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2004(t3057, t93920, t25460, t25698, t1071, t7150, t8521, t359, t42066, t3143, t36870, t1983);
    (t93821, t93884, t93890, t93893, t93897, t93904, t93920, t93921, t93928, t93963, t93968, t93983)
}
