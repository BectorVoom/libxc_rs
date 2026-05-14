//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 677/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk677<F: Float>(t3885: F, t9853: F, t2606: F, t9723: F, t9727: F, t9735: F, t9701: F, t9730: F, t9520: F, t9695: F, t9705: F, t9711: F, t9715: F, t9720: F, t9739: F, t9752: F) -> (F, F, F) {
    let t9854 = t3885 * t9853;
    let t9855 = t2606 * t9854;
    let t9861 = t9723 / 9.0;
    let t9862 = 2.0 / 27.0 * t9727;
    let t9863 = 4.0 / 27.0 * t9735;
    let t9867 = 4.0 / 9.0 * t9701;
    let t9869 = 2.0 / 3.0 * t9730;
    let t9870 = t9520 / 3.0;
    let t9871 = -t9705 / 9.0 + 2.0 * t9715 - 10.0 / 81.0 * t9720 + t9861 + t9862 - t9863 - 2.0 / 3.0 * t9739 + 4.0 / 9.0 * t9752 - t9695 / 3.0 - t9867 - 2.0 * t9711 - t9869 + t9870;
    (t9854, t9855, t9871)
}
