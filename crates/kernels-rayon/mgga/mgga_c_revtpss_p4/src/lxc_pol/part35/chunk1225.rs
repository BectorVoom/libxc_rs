//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1225/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1225(t115673: f64, t115687: f64, t115698: f64, t115712: f64, t103521: f64, t103547: f64, t110256: f64, t110639: f64, t110677: f64, t110679: f64, t1580: f64, t1956: f64, t1957: f64, t213: f64, t225: f64, t231: f64, t233: f64, t23414: f64, t257: f64, t27199: f64, t30396: f64, t30411: f64, t6016: f64, t6071: f64, t7070: f64, t7071: f64, t7076: f64, t7403: f64, t7997: f64, t95914: f64, t95930: f64) -> f64 {
    let t115714 = t115673 + t115687 + t115698 + t115712;
    let t115744 = 0.58544643236296698113e-1_f64 * t110639 + 0.65854491829355115987e0_f64 * t213 * t115714 * t225 * t257 + t95914 - t95930 - 0.4336814094102599731e0_f64 * t1956 * t1957 * t233 * t115714 - 0.19756347548806534796e1_f64 * t110256 * t1580 + 0.57824187921367996415e-1_f64 * t103521 - 0.39512695097613069591e1_f64 * t7403 * t23414 + 0.43368140941025997312e-1_f64 * t110677 - 0.77108554593144223218e-1_f64 * t110679 + 0.13010442282307799193e1_f64 * t7070 * t7076 * t7997 * t6016 * t231 + 0.13010442282307799193e1_f64 * t27199 * t30396 + 0.26020884564615598386e1_f64 * t7070 * t7071 * t7997 * t6071 - 0.78062653693846795158e1_f64 * t27199 * t30411 - 0.28912093960683998208e-1_f64 * t103547;
    t115744
}
