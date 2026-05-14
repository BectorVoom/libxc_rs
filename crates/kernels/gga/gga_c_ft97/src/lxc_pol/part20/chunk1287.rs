//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1287/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1287<F: Float>(t112647: F, t112649: F, t112654: F, t112657: F, t112713: F, t112766: F, t112811: F, t112849: F, t112903: F, t112950: F, t113006: F, t113660: F, t113709: F, t113764: F, t113802: F, t113840: F, t113895: F, t113947: F, t113983: F, t114037: F, t114078: F, t114132: F, t114150: F, t114181: F, t114227: F, t114263: F, t114529: F, t114570: F, t114621: F, t114665: F, t114669: F, t114719: F, t114762: F, t114806: F, t114853: F, t114894: F, t114936: F, t114981: F, t15477: F, t25448: F, t25459: F, t25471: F, t29008: F, t29026: F, t301: F, t44280: F, t6216: F, t6217: F, t98714: F, t98716: F) -> (F,) {
    let t114998 = -t112647 - t112649 - t98714 / 18.0 - t98716 / 9.0 - t25459 * t29026 / 9.0 + 4.0 * t112654 - 12.0 * t112657 - t301 * (t114078 + t114037 + t113983 + t113947 + t113895 + t113840 + t113802 + t113764 + t113709 + t113660 + t113006 + t112950 + t112903 + t112849 + t112811 + t112766 + t112713 + t114181 + t114669 + t114570 + t114263 + t114227 + t114981 + t114529 + t114894 + t114936 + t114621 + t114806 + t114762 + t114853 + t114719 + t114132) - 12.0 * t114665 + 8.0 * t114150 - t29008 * t25448 / 9.0 + t29008 * t25471 / 9.0 - 4.0 * t6216 * t44280 * t6217 * t15477;
    (t114998,)
}
