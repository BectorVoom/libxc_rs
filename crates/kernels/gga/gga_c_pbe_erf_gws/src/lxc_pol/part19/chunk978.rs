//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 978/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk978<F: Float>(t11048: F, t639: F, t10539: F, t7210: F, t3390: F, t626: F, t422: F, t4927: F, t3473: F, t617: F, t1809: F, t1620: F) -> (F, F, F, F) {
    let t11050 = F::cast_from(32.0_f64) / F::cast_from(81.0_f64) * t639 * t11048;
    let t11051 = t7210 * t10539;
    let t11053 = F::cast_from(16.0_f64) / F::cast_from(27.0_f64) * t639 * t11051;
    let t11054 = t3390 * t626;
    let t11055 = t11054 * t422;
    let t11056 = t4927 * t11055;
    let t11058 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t639 * t11056;
    let t11059 = t3473 * t617;
    let t11060 = t1809 * t11059;
    let t11062 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t1620 * t11060;
    (t11050, t11053, t11058, t11062)
}
