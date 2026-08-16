//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta657 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2610;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2611;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta657(t3495: f64, t6534: f64, t1189: f64, t1196: f64, t12552: f64, t6518: f64, t1187: f64, t12555: f64, t3520: f64, t5206: f64, t20571: f64, t20573: f64, t20576: f64, t20579: f64, t20582: f64, t20631: f64, t20633: f64, t20635: f64, t20637: f64, t20639: f64, t20643: f64, t20647: f64, t20650: f64, t20654: f64, t20690: f64, t20885: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t20886, t20887, t20889, t20890, t20892, t20894, t20895, t20896, t20898, t20899) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2610(t3495, t6534, t1189, t1196, t12552, t6518, t1187, t12555, t3520, t5206, t20571, t20573, t20576, t20579, t20582, t20631, t20633, t20635, t20637, t20639, t20643, t20647, t20650, t20654, t20690);
        let t20900 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2611(t20885, t20899);
    (t20886, t20887, t20889, t20890, t20892, t20894, t20895, t20896, t20898, t20900)
}
