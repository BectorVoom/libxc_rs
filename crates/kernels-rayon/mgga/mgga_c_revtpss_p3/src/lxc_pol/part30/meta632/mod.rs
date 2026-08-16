//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta632 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2198;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2199;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2200;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta632(t101435: f64, t2035: f64, t28196: f64, t28197: f64, t75365: f64, t94976: f64, t1513: f64, t94975: f64, t28036: f64, t94978: f64, t25823: f64, t4287: f64, t2340: f64, t94982: f64, t665: f64, t25826: f64, t2366: f64, t13509: f64, t6998: f64, t94974: f64, t94979: f64, t94981: f64, t114: f64, t508: f64, t651: f64, t530: f64, t7933: f64, t2014: f64, t25865: f64, t1353: f64, t22496: f64, t28167: f64, t8717: f64, t25082: f64, t73394: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t101436, t101439, t101448, t101451, t101454, t101455) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2198(t101435, t2035, t28196, t28197, t75365, t94976, t1513, t94975, t28036, t94978, t25823, t4287);
        let t101468 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2199(t101455, t1513, t2340, t94982, t4287, t665, t25826, t2366, t13509, t6998, t101448, t101451, t101454, t94974, t94979, t94981);
        let (t101469, t101472, t101476, t101482, t101485) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2200(t114, t101468, t508, t651, t530, t7933, t2014, t25865, t1353, t22496, t28167, t8717, t25082, t73394);
    (t101436, t101439, t101469, t101472, t101476, t101482, t101485)
}
