//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 812/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk812<F: Float>(t463: F, t488: F, t100: F, t370: F, t110: F, t8326: F, t1780: F, t480: F, t2: F, t8275: F, t1555: F, t26: F) -> (F, F, F, F, F, F, F) {
    let t11472 = t463 * t488;
    let t11490 = t370 * t100;
    let t11552 = t8326 * t110;
    let t11556 = t1780 * t488;
    let t11587 = t1780 * t480;
    let t11690 = t8275 * t2;
    let t11755 = t26 * t1555;
    (t11472, t11490, t11552, t11556, t11587, t11690, t11755)
}
