//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta631 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2196;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2197;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta631(t1493: f64, t2248: f64, t77: f64, t2315: f64, t2259: f64, t4173: f64, t38: f64, t60248: f64, t2251: f64, t28104: f64, t644: f64, t2014: f64, t25177: f64, t7934: f64, t28019: f64, t531: f64, t7238: f64, t25866: f64, t7898: f64, t13867: f64, t28167: f64, t8996: f64, t13872: f64, t13517: f64, t196: f64, t197: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t101337, t101350, t101357, t101360, t101376, t101399, t101416) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2196(t1493, t2248, t77, t2315, t2259, t4173, t38, t60248, t2251, t28104, t644, t2014, t25177, t7934);
        let (t101420, t101422, t101428, t101431, t101435) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2197(t28019, t531, t2014, t7238, t25866, t7898, t13867, t28167, t8996, t13872, t13517, t196, t197);
    (t101337, t101350, t101357, t101360, t101376, t101399, t101416, t101420, t101422, t101428, t101431, t101435)
}
