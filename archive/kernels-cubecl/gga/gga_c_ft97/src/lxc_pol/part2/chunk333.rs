//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 333/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk333<F: Float>(t1675: F, t1683: F, t41: F, t42: F, t78: F, t388: F, t4: F, t40: F, t39: F, t53: F, t11: F, t55: F) -> (F, F, F, F, F, F, F) {
    let t1685 = F::cast_from(0.44057546758024691357e0_f64) * t41 * t42 * t1675 + F::cast_from(0.18770038718167957794e-1_f64) * t1683;
    let t1686 = t78 * t1685;
    let t1687 = t388 * t1686;
    let t1689 = t40 * t4;
    let t1690 = t39 * t1689;
    let t1691 = t53 * t53;
    let t1692 = t11 * t1691;
    let t1693 = t55 * t55;
    (t1685, t1687, t1689, t1690, t1691, t1692, t1693)
}
