//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2287/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2287<F: Float>(t55353: F, t7769: F, t16521: F, t7467: F, t1873: F, t19534: F, t3941: F, t28017: F, t671: F, t20173: F, t28899: F, t1395: F, t5456: F) -> (F, F, F, F, F, F) {
    let t100917 = F::cast_from(54.0_f64) * t55353 * t7769;
    let t100921 = F::cast_from(27.0_f64) * t16521 * t7467;
    let t100924 = F::cast_from(27.0_f64) * t3941 * t1873 * t19534;
    let t100927 = F::cast_from(27.0_f64) * t3941 * t28017 * t671;
    let t100929 = F::cast_from(27.0_f64) * t20173 * t28899;
    let t100930 = t1395 * t5456;
    (t100917, t100921, t100924, t100927, t100929, t100930)
}
