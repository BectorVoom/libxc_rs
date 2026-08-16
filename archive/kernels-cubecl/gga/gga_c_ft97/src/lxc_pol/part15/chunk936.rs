//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 936/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk936<F: Float>(t1775: F, t20349: F, t20353: F, t1710: F, t20087: F, t1597: F, t19977: F, t7906: F, t3020: F, t58877: F, t938: F, t20040: F, t458: F) -> (F, F, F, F, F, F, F) {
    let t73675 = t1775 * t20349;
    let t73677 = t1775 * t20353;
    let t73718 = t1710 * t20087;
    let t73777 = t19977 * t1597;
    let t73906 = t7906 * t73777;
    let t73912 = t3020 * t58877 * t938;
    let t73956 = t458 * t20040;
    (t73675, t73677, t73718, t73777, t73906, t73912, t73956)
}
