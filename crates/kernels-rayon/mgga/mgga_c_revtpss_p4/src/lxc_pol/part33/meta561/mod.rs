//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta561 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1957;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1958;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta561(t30128: f64, t651: f64, t18245: f64, t1936: f64, t1501: f64, t1518: f64, t4248: f64, t7741: f64, t5920: f64, t93: f64, t7889: f64, t1312: f64, t30004: f64, t1937: f64, t7735: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t30130, t30137, t30138) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1957(t30128, t651, t18245, t1936, t1501, t1518);
        let (t30140, t30142, t30143, t30145, t30147, t30149, t30154, t30156, t30158) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1958(t1936, t30138, t4248, t7741, t5920, t93, t7889, t1312, t30004, t18245, t1937, t7735);
    (t30130, t30137, t30138, t30140, t30142, t30143, t30145, t30147, t30149, t30154, t30156, t30158)
}
