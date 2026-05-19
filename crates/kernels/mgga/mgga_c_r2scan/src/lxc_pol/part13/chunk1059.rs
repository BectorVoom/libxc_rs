//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1059/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1059<F: Float>(t37523: F, t1266: F, t2317: F, t3446: F, t3448: F, t3434: F, t3439: F, t6860: F, t875: F, t10993: F, t502: F, t6876: F) -> (F, F, F, F) {
    let t37524 = F::cast_from(0.63245127235888530833e-7_f64) * t37523;
    let t37527 = t3446 * t1266 * t2317 * t3448;
    let t37528 = F::cast_from(0.44715219694310041527e-2_f64) * t37527;
    let t37531 = t3434 * t6860 * t875 * t3439;
    let t37532 = F::cast_from(0.16432021104515675446e-2_f64) * t37531;
    let t37541 = t3446 * t502 * t6876 * t10993;
    (t37524, t37528, t37532, t37541)
}
