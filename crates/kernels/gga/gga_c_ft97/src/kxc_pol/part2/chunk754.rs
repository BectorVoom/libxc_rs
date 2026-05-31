//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 754/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk754<F: Float>(t11838: F, t83: F, t110: F, t358: F, t447: F, t8232: F, t955: F, t1882: F, t3227: F, t3291: F, t379: F, t1852: F, t463: F) -> (F, F, F, F, F, F) {
    let t11839 = t83 * t11838;
    let t11843 = t447 * t110 * t358;
    let t11846 = t8232 * t955;
    let t11849 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1882 * t3227;
    let t11851 = t447 * t3291 * t379;
    let t11854 = t463 * t1852;
    (t11839, t11843, t11846, t11849, t11851, t11854)
}
