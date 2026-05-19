//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 891/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk891<F: Float>(t2148: F, t8160: F, t6165: F, t1632: F, t2531: F, t551: F, t574: F, t2185: F, t910: F, t552: F, t2654: F, t1592: F) -> (F, F, F, F) {
    let t8161 = t2148 * t8160;
    let t8163 = F::cast_from(0.34930954652346593434e-1_f64) * t6165 * t8161;
    let t8165 = t551 * t1632 * t2531;
    let t8167 = F::cast_from(0.23115257973478049502e0_f64) * t574 * t8165;
    let t8170 = t910 * t2185;
    let t8172 = t551 * t552 * t8170;
    let t8176 = t551 * t1632 * t2654;
    let t8178 = F::cast_from(0.69345773920434148506e0_f64) * t1592 * t8176;
    (t8163, t8167, t8172, t8178)
}
