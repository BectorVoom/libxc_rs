//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 964/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk964<F: Float>(t41: F, t4879: F, t335: F, t333: F, t3110: F, t317: F, t319: F, t3072: F, t311: F, t10112: F, t313: F, t1031: F, t1068: F) -> (F, F, F, F, F, F) {
    let t10138 = t4879 * t41;
    let t10139 = t10138 * t335;
    let t10141 = F::cast_from(0.72818958333333333333e-4_f64) * t333 * t10139;
    let t10144 = F::cast_from(0.27323333333333333333e-1_f64) * t317 * t3110 * t319;
    let t10170 = F::new(1.0) / t3072 / t311;
    let t10187 = F::cast_from(0.14055920378328537299e-1_f64) * t10112 * t313;
    let t10188 = t1068 * t1031;
    (t10138, t10141, t10144, t10170, t10187, t10188)
}
