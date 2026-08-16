//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1311/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1311<F: Float>(t16693: F, t20749: F, t46376: F, t16689: F, t5597: F, t39585: F, t39590: F, t39593: F, t41254: F, t75943: F, t75950: F, t75951: F, t75952: F) -> (F, F, F, F) {
    let t76017 = F::cast_from(144.0_f64) * t16693 * t20749;
    let t76018 = F::cast_from(0.23392894490538584828e1_f64) * t46376;
    let t76020 = F::cast_from(24.0_f64) * t16689 * t5597;
    let t76021 = t75943 - t39585 + t39590 - t39593 + t75950 + t75951 - t75952 + t76017 + t41254 - t76018 + t76020;
    (t76017, t76018, t76020, t76021)
}
