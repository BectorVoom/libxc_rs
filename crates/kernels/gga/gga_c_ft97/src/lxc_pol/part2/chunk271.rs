//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 271/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk271<F: Float>(t1017: F, t586: F, t24: F, t1033: F, t462: F, t581: F, t92: F, t579: F, t91: F, t1000: F, t1020: F, t594: F) -> (F, F, F, F) {
    let t1036 = t586 * t1017;
    let t1037 = t24 * t1036;
    let t1039 = -t581 - t462 * t1033 / F::cast_from(3.0_f64) - t92 * t1037;
    let t1041 = t91 * t579 * t1039;
    let t1045 = t1041 / F::cast_from(6.0_f64) - t594 - t1000 / F::cast_from(9.0_f64) - t1020 / F::cast_from(3.0_f64);
    (t1037, t1039, t1041, t1045)
}
