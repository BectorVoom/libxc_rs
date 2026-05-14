//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 328/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk328<F: Float>(t397: F, t53: F, t428: F, t1701: F, t407: F, t76: F, t66: F, t14: F, t1675: F, t68: F, t72: F, t172: F, t391: F, t67: F, t9: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t1702 = t53 * t397;
    let t1703 = t1702 * t428;
    let t1704 = t1701 * t1703;
    let t1710 = 1.0 / t407 / t76;
    let t1711 = t66 * t1710;
    let t1712 = t428 * t428;
    let t1713 = t1711 * t1712;
    let t1716 = t1675 * t14;
    let t1718 = t68 * t1716 * t72;
    let t1720 = t391 * t172;
    let t1722 = t68 * t1720 * t72;
    let t1725 = t9 * t67 * t391;
    (t1702, t1703, t1704, t1710, t1711, t1712, t1713, t1718, t1722, t1725)
}
