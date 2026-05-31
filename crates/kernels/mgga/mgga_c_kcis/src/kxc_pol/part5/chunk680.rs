//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 680/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk680<F: Float>(t1064: F, t4581: F, t4567: F, t945: F, t1079: F, t1056: F, t345: F, t104: F, t111: F, t120: F, t3061: F, t3105: F, t3109: F, t3113: F, t3114: F, t3122: F, t3130: F, t3150: F, t4547: F) -> (F, F, F, F, F, F, F) {
    let t4898 = t1064 * t4581;
    let t4901 = t945 * t4567;
    let t4904 = t1079 * t4581;
    let t4907 = t1056 * t4567;
    let t4910 = t1056 * t4581;
    let t4913 = t345 * t4567;
    let t4919 = -F::cast_from(0.23911438650126355246e-1_f64) * t3061 * t4547 + F::cast_from(0.15538616723388920628e-3_f64) * t3150 * t4547 - F::cast_from(0.1585e-2_f64) * t111 * t4898 - F::cast_from(0.52833333333333333333e-3_f64) * t111 * t4901 - F::cast_from(0.10082625e-4_f64) * t120 * t4904 - F::cast_from(0.672175e-5_f64) * t120 * t4907 + F::cast_from(0.7026e-2_f64) * t104 * t4910 + F::cast_from(0.1171e-2_f64) * t104 * t4913 + t3105 - t3109 - t3113 + F::cast_from(0.4684e-2_f64) * t3114 - F::cast_from(0.13208333333333333333e-2_f64) * t3122 - F::cast_from(0.117630625e-4_f64) * t3130;
    (t4898, t4901, t4904, t4907, t4910, t4913, t4919)
}
