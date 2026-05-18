//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 242/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk242<F: Float>(t265: F, t735: F, t256: F, t267: F, t566: F, t581: F, t585: F, t595: F, t614: F, t621: F, t635: F, t638: F, t647: F, t665: F, t708: F, t716: F, t722: F, t725: F, t732: F) -> (F, F) {
    let t737 = F::new(2.0) / F::new(45.0) * t265 * t735;
    let t738 = t566 + t581 + t585 + t595 - t614 + t621 + t635 + t638 + t647 - t665 + t708 * t256 / F::new(3.0) + t716 + t722 + t725 - t732 * t267 / F::new(15.0) - t737;
    (t737, t738)
}
