//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 353/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk353<F: Float>(t340: F, t984: F, t343: F, t974: F, t346: F, t964: F, t971: F, t973: F, t980: F) -> (F, F, F) {
    let t985 = t340 * t984;
    let t986 = t985 * t343;
    let t987 = t974 * t986;
    let t990 = -F::cast_from(0.22222222222222222222e-2_f64) * t964 * t346 + t971 + F::cast_from(0.27777777777777777777e-3_f64) * t973 * t980 - F::cast_from(0.83333333333333333332e-3_f64) * t973 * t987;
    (t986, t987, t990)
}
