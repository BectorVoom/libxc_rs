//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1153/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1153<F: Float>(t16535: F, t1873: F, t6534: F, t671: F, t3941: F, t2363: F, t1401: F, t22479: F, t2274: F, t50: F, t2244: F, t2250: F, t22510: F, t7251: F) -> (F, F, F, F, F, F, F, F) {
    let t23892 = F::cast_from(27.0_f64) * t16535 * t1873;
    let t23893 = t6534 * t671;
    let t23895 = F::cast_from(54.0_f64) * t3941 * t23893;
    let t23896 = t1873 * t2363;
    let t23898 = F::cast_from(27.0_f64) * t3941 * t23896;
    let t23900 = F::cast_from(0.135e2_f64) * t1401 * t22479;
    let t24498 = t50 * t2274;
    let t24503 = F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t24498 * t2244 - F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t7251 * t2250 - t22510;
    (t23892, t23893, t23895, t23896, t23898, t23900, t24498, t24503)
}
