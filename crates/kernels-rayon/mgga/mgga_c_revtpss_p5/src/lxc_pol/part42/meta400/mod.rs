//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta400 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1359;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1360;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1361;
use chunk3::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1362;
use chunk4::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1363;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta400(t1248: f64, t6573: f64, t1250: f64, t3720: f64, t19666: f64, t5302: f64, t1042: f64, t17550: f64, t19661: f64, t1715: f64, t17500: f64, t5056: f64, t5277: f64, t20261: f64, t20263: f64, t20386: f64, t20388: f64, t20390: f64, t20393: f64, t20396: f64, t20399: f64, t20402: f64, t20404: f64, t20450: f64, t20452: f64, t20454: f64, t20471: f64, t20475: f64, t20477: f64, t20685: f64, t3495: f64, t6534: f64, t1189: f64, t1196: f64, t12552: f64, t6518: f64, t1187: f64, t12555: f64, t3520: f64, t5206: f64, t20571: f64, t20573: f64, t20576: f64, t20579: f64, t20582: f64, t20631: f64, t20633: f64, t20635: f64, t20637: f64, t20639: f64, t20643: f64, t20647: f64, t20650: f64, t20654: f64, t20690: f64, t482: f64, t19680: f64, t5268: f64, t1247: f64, t1261: f64, t12910: f64, t12956: f64, t17339: f64, t17396: f64, t17505: f64, t3708: f64, t3711: f64, t5299: f64, t5354: f64, t6619: f64, t6625: f64, t20823: f64, t5265: f64, t5274: f64, t1774: f64, t3362: f64, t4181: f64, t12787: f64, t12916: f64, t6689: f64, t3718: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t20856, t20858, t20864, t20868, t20876, t20879) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1359(t1248, t6573, t1250, t3720, t19666, t5302, t1042, t17550, t19661, t1715, t17500, t5056, t5277);
        let (t20880, t20885) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1360(t1042, t20879, t20261, t20263, t20386, t20388, t20390, t20393, t20396, t20399, t20402, t20404, t20450, t20452, t20454, t20471, t20475, t20477, t20685);
        let (t20889, t20894, t20898, t20899) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1361(t3495, t6534, t1189, t1196, t12552, t6518, t1187, t12555, t3520, t5206, t20571, t20573, t20576, t20579, t20582, t20631, t20633, t20635, t20637, t20639, t20643, t20647, t20650, t20654, t20690);
        let (t20900, t20910) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1362(t20885, t20899, t1250, t482, t1042, t19680, t5268, t1247, t1261, t12910, t12956, t17339, t17396, t17505, t20858, t20864, t20868, t20876, t20880, t3708, t3711, t5299, t5354, t6619, t6625);
        let (t20914, t20917, t20923, t20927) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1363(t20823, t5268, t1042, t5265, t5274, t1774, t3362, t4181, t12787, t12916, t6689, t3718);
    (t20856, t20889, t20894, t20898, t20900, t20910, t20914, t20917, t20923, t20927)
}
