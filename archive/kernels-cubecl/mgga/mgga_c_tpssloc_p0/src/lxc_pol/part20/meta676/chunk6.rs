//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2556/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2556<F: Float>(t300: F, t51381: F, t51411: F, t51450: F, t51493: F, t51538: F, t51617: F, t51664: F, t51789: F, t15041: F, t3411: F, t11126: F, t4884: F) -> (F, F, F) {
    let t51793 = t300 * (t51381 + t51411 + t51450 + t51493 + t51538 + t51617 + t51664 + t51789);
    let t51795 = F::cast_from(0.51947577317044391277e2_f64) * t3411 * t15041;
    let t51797 = F::cast_from(0.51947577317044391277e2_f64) * t11126 * t4884;
    (t51793, t51795, t51797)
}
