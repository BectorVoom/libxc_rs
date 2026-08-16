//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 841/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk841<F: Float>(t1055: F, t8232: F, t1882: F, t3548: F, t1060: F, t1986: F, t2185: F, t3575: F, t167: F, t358: F, t569: F, t1030: F) -> (F, F, F, F, F, F) {
    let t13187 = t8232 * t1055;
    let t13190 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1882 * t3548;
    let t13192 = t2185 * t1060 * t1986;
    let t13196 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1882 * t3575;
    let t13198 = t569 * t167 * t358;
    let t13201 = t8232 * t1030;
    (t13187, t13190, t13192, t13196, t13198, t13201)
}
