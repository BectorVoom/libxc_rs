//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 339/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk339<F: Float>(t1711: F, t1712: F, t14: F, t1675: F, t68: F, t72: F, t172: F, t391: F, t67: F, t9: F) -> (F, F, F, F) {
    let t1713 = t1711 * t1712;
    let t1716 = t1675 * t14;
    let t1718 = t68 * t1716 * t72;
    let t1720 = t391 * t172;
    let t1722 = t68 * t1720 * t72;
    let t1725 = t9 * t67 * t391;
    (t1713, t1718, t1722, t1725)
}
