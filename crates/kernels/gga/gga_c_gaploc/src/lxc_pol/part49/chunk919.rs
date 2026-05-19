//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 919/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk919<F: Float>(t2859: F, t3137: F, t4752: F, t10557: F, t9431: F, t2487: F, t41749: F, t6711: F, t41810: F, t6710: F, t3338: F, t874: F) -> (F, F, F, F, F) {
    let t41829 = F::cast_from(0.7150097990370085334e0_f64) * t2859 * t4752 * t3137;
    let t41831 = F::cast_from(0.42900587942220512003e1_f64) * t10557 * t9431;
    let t41834 = F::cast_from(0.87421871174939309262e2_f64) * t2487 * t6711 * t41749;
    let t41837 = F::cast_from(0.11502877786176224903e2_f64) * t6710 * t6711 * t41810;
    let t41838 = t3338 * t874;
    (t41829, t41831, t41834, t41837, t41838)
}
