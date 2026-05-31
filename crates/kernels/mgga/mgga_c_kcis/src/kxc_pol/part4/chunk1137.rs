//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1137/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1137<F: Float>(t278: F, t1000: F, t992: F, t1071: F, t1003: F, t1646: F, t829: F, t2887: F, t2844: F, t14051: F, t1001: F, t286: F, t110: F, t1705: F) -> (F, F, F, F) {
    let t288 = F::cast_from(0.0_f64) < t278;
    let t14400 = t992 * t1000;
    let t14401 = t14400 * t1071;
    let t14402 = t1646 * t1003;
    let t14403 = t14402 * t829;
    let t14404 = t14401 * t14403;
    let t14407 = t2887 * t1000;
    let t14408 = t14407 * t2844;
    let t14409 = t14408 * t14403;
    let t14413 = piecewise3::<F>(t288, t14051, -t14051);
    let t14414 = t1001 * t14413;
    let t14415 = t286 * t14414;
    let t14422 = t110 * t1705;
    (t14404, t14409, t14415, t14422)
}
