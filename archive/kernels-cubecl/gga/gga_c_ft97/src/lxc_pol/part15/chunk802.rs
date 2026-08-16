//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 802/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk802<F: Float>(t21708: F, t21716: F, t241: F, t258: F, t21369: F, t265: F, t724: F, t10024: F, t21351: F, t1091: F, t5181: F, t1175: F, t4973: F) -> (F, F, F, F, F, F) {
    let t21717 = t21708 + t21716;
    let t21719 = t241 * t21717 * t258;
    let t21724 = t724 * t265 * t21369;
    let t21728 = t10024 * t265 * t21351;
    let t21732 = t724 * t5181 * t1091;
    let t21736 = t724 * t1175 * t4973;
    (t21717, t21719, t21724, t21728, t21732, t21736)
}
