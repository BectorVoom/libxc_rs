//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 602/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk602<F: Float>(t1056: F, t6334: F, t345: F, t6326: F, t1030: F, t104: F, t1072: F, t3105: F, t3109: F, t3113: F, t4869: F, t4871: F, t4885: F, t4887: F, t6276: F) -> (F, F, F) {
    let t6436 = t1056 * t6334;
    let t6439 = t345 * t6326;
    let t6450 = t3105 - t3109 - t3113 - F::cast_from(0.3513e-2_f64) * t104 * t6436 + F::cast_from(0.1171e-2_f64) * t104 * t6439 + F::cast_from(0.11955719325063177623e-1_f64) * t1030 * t6276 - F::cast_from(0.5179538907796306876e-4_f64) * t1072 * t6276 - F::cast_from(0.23911438650126355246e-1_f64) * t4869 + F::cast_from(0.20718155631185227504e-3_f64) * t4871 - F::cast_from(0.26416666666666666666e-2_f64) * t4885 - F::cast_from(0.23526125e-4_f64) * t4887;
    (t6436, t6439, t6450)
}
