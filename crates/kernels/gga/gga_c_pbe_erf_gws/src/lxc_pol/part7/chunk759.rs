//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 759/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk759<F: Float>(t6710: F, t822: F, t2150: F, t2273: F, t2319: F, t2332: F, t899: F, t900: F, t907: F, t2277: F, t6656: F, t6663: F, t6667: F, t6676: F, t6682: F, t6685: F, t6688: F, t6692: F, t6696: F, t6700: F, t6704: F, t6709: F) -> (F, F, F, F) {
    let t6711 = t822 * t6710;
    let t6713 = t6711 * t2150 / 16.0;
    let t6714 = t2319 * t2273;
    let t6717 = t899 * t900 * t2332;
    let t6718 = t6717 * t907;
    let t6720 = -35.0 / 384.0 * t6656 - t6663 + t2277 * t6667 / 768.0 + t6676 + t6682 + 3.0 / 256.0 * t6685 * t6688 + t6692 - t6696 - t6700 - t6704 - t6709 - t6713 - 7.0 / 768.0 * t6714 + 119.0 / 2304.0 * t6718;
    (t6711, t6713, t6717, t6720)
}
