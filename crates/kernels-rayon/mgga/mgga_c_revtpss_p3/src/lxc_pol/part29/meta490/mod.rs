//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta490 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1778;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1779;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta490(t72: f64, t8015: f64, t686: f64, t7058: f64, t7064: f64, t689: f64, t8011: f64, t25431: f64, t25411: f64, t786: f64, t7998: f64, t789: f64, t231: f64, t7997: f64, t836: f64, t7076: f64, t1558: f64, t7398: f64, t1580: f64, t7384: f64, t213: f64, t25383: f64, t26498: f64, t26500: f64, t26547: f64, t7067: f64, t7070: f64, t8012: f64, t8016: f64, t887: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t28359, t28360, t28361, t28366, t28368, t28369, t28371, t28373, t28374) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1778(t72, t8015, t686, t7058, t7064, t689, t8011, t25431, t25411, t786, t7998, t789);
        let (t28377, t28378, t28384, t28385, t28390, t28394, t28397) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1779(t231, t7997, t836, t7076, t1558, t7398, t1580, t7384, t689, t213, t25383, t26498, t26500, t26547, t28361, t28366, t28369, t28371, t28374, t7067, t7070, t8012, t8016, t887);
    (t28359, t28360, t28368, t28373, t28377, t28378, t28384, t28385, t28390, t28394, t28397)
}
