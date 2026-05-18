//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 794/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk794<F: Float>(t1882: F, t2811: F, t2807: F, t2682: F, t2862: F, t882: F, t295: F, t9568: F, t319: F, t9572: F, t2844: F, t824: F) -> (F, F, F, F, F, F) {
    let t10750 = t1882 * t2811;
    let t10752 = t1882 * t2807;
    let t10755 = t2862 * t882 * t2682;
    let t10758 = t9568 * t295;
    let t10760 = t10758 * t319 * t9572;
    let t10763 = t2844 * t824;
    (t10750, t10752, t10755, t10758, t10760, t10763)
}
