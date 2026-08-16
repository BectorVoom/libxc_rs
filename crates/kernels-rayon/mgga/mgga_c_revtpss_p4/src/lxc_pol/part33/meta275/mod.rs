//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta275 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1230;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta275(t7301: f64, t7925: f64, t545: f64, t7910: f64, t2028: f64, t1904: f64, t2027: f64, t2030: f64, t213: f64, t561: f64, t7245: f64, t7248: f64, t7279: f64, t7288: f64, t7291: f64, t7295: f64, t7911: f64, t7917: f64, t7921: f64) -> (f64, f64, f64, f64) {
        let (t7926, t7929, t7930, t7933) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1230(t7301, t7925, t545, t7910, t2028, t1904, t2027, t2030, t213, t561, t7245, t7248, t7279, t7288, t7291, t7295, t7911, t7917, t7921);
    (t7926, t7929, t7930, t7933)
}
