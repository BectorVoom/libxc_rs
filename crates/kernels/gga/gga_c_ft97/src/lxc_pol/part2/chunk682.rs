//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 682/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk682<F: Float>(t295: F, t9567: F, t2783: F, t458: F, t8282: F, t849: F, t1775: F, t2778: F, t2767: F, t303: F, t3051: F, t1771: F, t854: F) -> (F, F, F, F, F, F, F) {
    let t10580 = t9567 * t295;
    let t10584 = t458 * t2783;
    let t10586 = t8282 * t849;
    let t10589 = t1775 * t2778;
    let t10591 = t1775 * t2767;
    let t10594 = F::new(28.0) / F::new(27.0) * t3051 * t303;
    let t10595 = t1771 * t854;
    (t10580, t10584, t10586, t10589, t10591, t10594, t10595)
}
