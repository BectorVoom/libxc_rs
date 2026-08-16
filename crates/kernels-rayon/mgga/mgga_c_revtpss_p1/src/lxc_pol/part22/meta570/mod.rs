//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta570 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2418;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2419;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2420;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta570(t5962: f64, t854: f64, t236: f64, t807: f64, t2476: f64, t5966: f64, t10717: f64, t10719: f64, t10723: f64, t10746: f64, t10749: f64, t14780: f64, t14783: f64, t14817: f64, t14820: f64, t14823: f64, t45: f64, t57: f64, t5819: f64, t633: f64, t5825: f64, t80: f64, t18281: f64, t4186: f64, t4328: f64, t606: f64, t766: f64, t637: f64, t83: f64, t4335: f64, t770: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18348, t18349, t18352, t18353, t18361) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2418(t5962, t854, t236, t807, t2476, t5966, t10717, t10719, t10723, t10746, t10749, t14780, t14783, t14817, t14820, t14823);
        let (t18367, t18372, t18378, t18379, t18384, t18390) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2419(t45, t57, t5819, t633, t5825, t80, t18281, t4186, t4328, t606, t766, t637, t83, t4335, t770, zeta_threshold);
        let t18392 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2420(t18378, t18390);
    (t18348, t18349, t18352, t18353, t18361, t18367, t18372, t18379, t18384, t18392)
}
