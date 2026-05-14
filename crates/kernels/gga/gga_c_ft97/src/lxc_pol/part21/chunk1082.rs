//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1082/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1082<F: Float>(t26791: F, t378: F, t1984: F, t26768: F, t23405: F, t26823: F, t165: F, t6584: F, t94329: F, t26785: F, t24094: F, t6580: F, t26581: F, t5769: F, t26801: F, t358: F, t614: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t104161 = t378 * t26791;
    let t104175 = t1984 * t26768;
    let t104204 = t23405 * t26823 / 27.0;
    let t104205 = t26768 * t165;
    let t104213 = t94329 * t6584 / 27.0;
    let t104217 = t23405 * t26785 / 27.0;
    let t104220 = t6580 * t24094 / 9.0;
    let t104225 = t26581 * t5769 / 9.0;
    let t104252 = 2.0 / 27.0 * t23405 * t26801;
    let t104265 = t614 * t358;
    (t104161, t104175, t104204, t104205, t104213, t104217, t104220, t104225, t104252, t104265)
}
