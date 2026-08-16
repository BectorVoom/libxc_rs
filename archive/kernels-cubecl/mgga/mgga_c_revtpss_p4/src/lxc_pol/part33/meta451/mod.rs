//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta451 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1640;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1641;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1642;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta451<F: Float>(t1042: F, t20879: F, t20261: F, t20263: F, t20386: F, t20388: F, t20390: F, t20393: F, t20396: F, t20399: F, t20402: F, t20404: F, t20450: F, t20452: F, t20454: F, t20471: F, t20475: F, t20477: F, t20685: F, t3495: F, t6534: F, t1189: F, t1196: F, t12552: F, t6518: F, t1187: F, t12555: F, t3520: F, t5206: F, t20571: F, t20573: F, t20576: F, t20579: F, t20582: F, t20631: F, t20633: F, t20635: F, t20637: F, t20639: F, t20643: F, t20647: F, t20650: F, t20654: F, t20690: F, t1250: F, t482: F, t19680: F, t5268: F, t1247: F, t1261: F, t12910: F, t12956: F, t17339: F, t17396: F, t17505: F, t20858: F, t20864: F, t20868: F, t20876: F, t3708: F, t3711: F, t5299: F, t5354: F, t6619: F, t6625: F) -> (F, F, F, F, F, F, F, F) {
        let (t20880, t20885) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1640::<F>(t1042, t20879, t20261, t20263, t20386, t20388, t20390, t20393, t20396, t20399, t20402, t20404, t20450, t20452, t20454, t20471, t20475, t20477, t20685);
        let (t20889, t20894, t20898, t20899) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1641::<F>(t3495, t6534, t1189, t1196, t12552, t6518, t1187, t12555, t3520, t5206, t20571, t20573, t20576, t20579, t20582, t20631, t20633, t20635, t20637, t20639, t20643, t20647, t20650, t20654, t20690);
        let (t20900, t20903, t20907, t20910) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1642::<F>(t20885, t20899, t1250, t482, t1042, t19680, t5268, t1247, t1261, t12910, t12956, t17339, t17396, t17505, t20858, t20864, t20868, t20876, t20880, t3708, t3711, t5299, t5354, t6619, t6625);
    (t20880, t20889, t20894, t20898, t20900, t20903, t20907, t20910)
}
