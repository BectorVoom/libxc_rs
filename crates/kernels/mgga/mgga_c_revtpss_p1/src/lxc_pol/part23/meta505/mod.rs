//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta505 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1996;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1997;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta505<F: Float>(t3495: F, t6534: F, t1189: F, t1196: F, t12552: F, t6518: F, t1187: F, t12555: F, t3520: F, t5206: F, t20571: F, t20573: F, t20576: F, t20579: F, t20582: F, t20631: F, t20633: F, t20635: F, t20637: F, t20639: F, t20643: F, t20647: F, t20650: F, t20654: F, t20690: F, t20885: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t20887, t20889, t20890, t20891, t20892, t20894, t20895, t20896, t20898, t20899) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1996::<F>(t3495, t6534, t1189, t1196, t12552, t6518, t1187, t12555, t3520, t5206, t20571, t20573, t20576, t20579, t20582, t20631, t20633, t20635, t20637, t20639, t20643, t20647, t20650, t20654, t20690);
        let t20900 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1997::<F>(t20885, t20899);
    (t20887, t20889, t20890, t20891, t20892, t20894, t20895, t20896, t20898, t20900)
}
