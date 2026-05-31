//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1052/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1052<F: Float>(t12335: F, t2253: F, t12338: F, t7943: F, t4184: F, t7962: F, t4190: F, t12345: F, t1555: F, t4189: F, t4310: F, t4244: F, t573: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t27498 = t12335 * t2253;
    let t27500 = F::cast_from(4.0_f64) * t12338 * t7943;
    let t27502 = F::cast_from(2.0_f64) * t4184 * t7962;
    let t27503 = t2253 * t4190;
    let t27505 = F::cast_from(6.0_f64) * t12345 * t27503;
    let t27506 = t7962 * t1555;
    let t27508 = F::cast_from(4.0_f64) * t4189 * t27506;
    let t27509 = t2253 * t4310;
    let t27511 = F::cast_from(2.0_f64) * t4189 * t27509;
    let t27512 = t4244 * t573;
    (t27498, t27500, t27502, t27503, t27505, t27506, t27508, t27509, t27511, t27512)
}
