//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta207 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk942;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk943;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk944;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk945;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta207<F: Float>(t2475: F, t72: F, t245: F, t2482: F, t814: F, t823: F, t136: F, t853: F, t220: F, t820: F, t844: F, t2681: F, t222: F, t9727: F, t2737: F, t9802: F, t596: F, t27: F, t2719: F, t843: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t10769, t10770, t10777) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk942::<F>(t2475, t72, t245, t2482, t814, t823);
        let t10779 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk943::<F>(t136, t853, t220);
        let t10811 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk944::<F>(t820, t823, t844);
        let (t10815, t10824, t10826, t10845, t10850, t10858) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk945::<F>(t2681, t820, t823, t222, t9727, t2737, t9802, t2482, t596, t27, t2719, t843);
    (t10769, t10770, t10777, t10779, t10811, t10815, t10824, t10826, t10845, t10850, t10858)
}
