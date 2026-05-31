//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1257/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1257<F: Float>(t158: F, t3128: F, t3446: F, t3447: F, t874: F, t122: F, t3434: F, t3437: F, t10619: F, t12567: F, t3262: F, t3264: F, t42444: F) -> (F, F, F, F) {
    let t43936 = t158 * t3128;
    let t43939 = t3446 * t3447 * t43936 * t874;
    let t43943 = t3434 * t3437 * t43936 * t122;
    let t43946 = t12567 * t10619 / F::cast_from(4.0_f64);
    let t43949 = F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t3262 * t42444 * t3264;
    (t43939, t43943, t43946, t43949)
}
