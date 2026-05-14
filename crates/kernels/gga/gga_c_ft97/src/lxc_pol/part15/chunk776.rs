//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 776/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk776<F: Float>(t487: F, t7763: F, t100: F, t38477: F, t1786: F, t1852: F, t488: F, t8216: F, t8326: F, t38463: F, t38052: F, t82: F, t38482: F, t104: F, t38061: F, t89: F) -> (F, F, F, F, F, F, F, F, F) {
    let t39021 = t487 * t7763;
    let t39026 = t38477 * t100;
    let t39107 = t1786 * t1852;
    let t39120 = t8216 * t488;
    let t39167 = t8326 * t488;
    let t39230 = t38463 * t100;
    let t39243 = t38052 * t82;
    let t39272 = t38482 * t100;
    let t39317 = 280.0 / 243.0 * t89 * t38061 * t104;
    (t39021, t39026, t39107, t39120, t39167, t39230, t39243, t39272, t39317)
}
