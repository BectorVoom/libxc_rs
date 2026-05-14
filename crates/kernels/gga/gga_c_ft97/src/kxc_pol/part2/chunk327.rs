//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 327/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk327<F: Float>(t1675: F, t1683: F, t41: F, t42: F, t78: F, t388: F, t4: F, t40: F, t39: F, t53: F, t11: F, t55: F, t45: F, t12: F, t51: F) -> (F, F, F, F, F, F, F) {
    let t1685 = 0.44057546758024691357e0 * t41 * t42 * t1675 + 0.18770038718167957794e-1 * t1683;
    let t1686 = t78 * t1685;
    let t1687 = t388 * t1686;
    let t1689 = t40 * t4;
    let t1690 = t39 * t1689;
    let t1691 = t53 * t53;
    let t1692 = t11 * t1691;
    let t1693 = t55 * t55;
    let t1696 = 1.0 / t45 / t1693 / t55;
    let t1697 = t1692 * t1696;
    let t1698 = t1690 * t1697;
    let t1701 = t51 * t12;
    (t1685, t1687, t1689, t1691, t1696, t1698, t1701)
}
