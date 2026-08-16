//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta133 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk701;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk702;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk703;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk704;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk705;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta133<F: Float>(t1794: F, t3153: F, t3767: F, t5330: F, t73: F, t140: F, t1781: F, t1222: F, t127: F, t1789: F, t371: F, t1235: F, t1219: F, t1778: F, t1010: F, t1480: F, t1715: F, t3634: F, t247: F, t1261: F, t1260: F, t1785: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t5332, t5340) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk701::<F>(t1794, t3153, t3767, t5330);
        let (t5351, t5357, t5358, t5362) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk702::<F>(t1794, t73, t140, t1781, t1222, t127, t1789, t371);
        let (t5363, t5366, t5373) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk703::<F>(t1235, t5362, t1219, t1778, t1010, t1480);
        let t5378 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk704::<F>(t1715, t3634, t247);
        let (t5379, t5381) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk705::<F>(t1261, t5378, t1260, t1785);
    (t5332, t5340, t5351, t5357, t5358, t5362, t5363, t5366, t5373, t5378, t5379, t5381)
}
