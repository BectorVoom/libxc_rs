//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta852 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2993;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2994;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta852(t4186: f64, t4401: f64, t606: f64, t749: f64, t14362: f64, t9575: f64, t123: f64, t2630: f64, t4392: f64, t4398: f64, t9318: f64, t15071: f64, t892: f64, t14322: f64, t2516: f64, t2496: f64, t14426: f64, t177: f64, t762: f64, t10428: f64, t4305: f64, t2609: f64, t706: f64, t10436: f64, t4311: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t49911, t49926, t49929, t49940, t49950) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2993(t4186, t4401, t606, t749, t14362, t9575, t123, t2630, t4392, t4398, t9318, t15071, t892);
        let (t49957, t49963, t49966, t49978, t49981, t49983) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2994(t14322, t2516, t2496, t14426, t177, t762, t10428, t4305, t2609, t4186, t706, t10436, t4311);
    (t49911, t49926, t49929, t49940, t49950, t49957, t49963, t49966, t49978, t49981, t49983)
}
