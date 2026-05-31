//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 496/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk496<F: Float>(t2899: F, t332: F, t5: F, t885: F, t170: F, t2248: F, t328: F, t2253: F, t895: F, t906: F, t70: F, t703: F) -> (F, F, F, F, F, F) {
    let t2900 = t2899 * t332;
    let t2904 = t5 * t885;
    let t2912 = F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t170 * t2248 * t328;
    let t2913 = t2253 * t895;
    let t2915 = t2253 * t906;
    let t2917 = t70 * t703;
    (t2900, t2904, t2912, t2913, t2915, t2917)
}
