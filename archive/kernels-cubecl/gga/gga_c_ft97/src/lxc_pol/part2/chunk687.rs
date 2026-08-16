//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 687/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk687<F: Float>(t1882: F, t2846: F, t2899: F, t5: F, t2253: F, t2953: F, t170: F, t328: F, t8715: F, t8640: F, t906: F, t2925: F, t8675: F) -> (F, F, F, F, F, F) {
    let t10804 = t1882 * t2846;
    let t10829 = t5 * t2899;
    let t10835 = t2253 * t2953;
    let t10838 = F::cast_from(20.0_f64) / F::cast_from(27.0_f64) * t170 * t8715 * t328;
    let t10839 = t8640 * t906;
    let t10841 = t8675 * t2925;
    (t10804, t10829, t10835, t10838, t10839, t10841)
}
