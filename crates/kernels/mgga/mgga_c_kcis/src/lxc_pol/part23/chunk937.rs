//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 937/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk937<F: Float>(t17357: F, t6027: F, t12505: F, t2039: F, t2062: F, t4278: F, t12568: F, t5919: F, t12530: F, t5916: F, t2051: F, t4303: F) -> (F, F, F, F, F, F) {
    let t17358 = t6027 * t17357;
    let t17360 = t12505 * t2039;
    let t17362 = t4278 * t2062;
    let t17364 = t12568 * t5919;
    let t17366 = t12530 * t5916;
    let t17368 = t2051 * t4303;
    (t17358, t17360, t17362, t17364, t17366, t17368)
}
