//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 617/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk617<F: Float>(t143: F, t5623: F, t1317: F, t562: F, t543: F, t1478: F, t1483: F, t1507: F, t1991: F, t1995: F, t2018: F, t4202: F, t545: F, t5459: F, t5464: F, t5482: F, t5494: F, t5499: F, t5527: F) -> (F, F, F, F) {
    let t5938 = t5623 * t143;
    let t5947 = t562 * t1317;
    let t5958 = t562 * t543;
    let t5963 = F::cast_from(0.619125e-2_f64) * t5938 * t545 + F::cast_from(0.9286875e-2_f64) * t2018 * t1478 - F::cast_from(0.619125e-2_f64) * t2018 * t1483 + F::cast_from(0.9286875e-2_f64) * t1507 * t1991 + F::cast_from(0.46434375e-2_f64) * t5947 * t5459 - F::cast_from(0.9286875e-2_f64) * t4202 * t5464 + F::cast_from(0.9286875e-2_f64) * t562 * t5482 - F::cast_from(0.619125e-2_f64) * t1507 * t1995 - F::cast_from(0.9286875e-2_f64) * t4202 * t5494 + F::cast_from(0.123825e-1_f64) * t5958 * t5499 - F::cast_from(0.619125e-2_f64) * t562 * t5527;
    (t5938, t5947, t5958, t5963)
}
