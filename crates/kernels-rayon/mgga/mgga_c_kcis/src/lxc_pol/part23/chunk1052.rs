//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1052/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1052(t12335: f64, t2253: f64, t12338: f64, t7943: f64, t4184: f64, t7962: f64, t4190: f64, t12345: f64, t1555: f64, t4189: f64, t4310: f64, t4244: f64, t573: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t27498 = t12335 * t2253;
    let t27500 = 4.0_f64 * t12338 * t7943;
    let t27502 = 2.0_f64 * t4184 * t7962;
    let t27503 = t2253 * t4190;
    let t27505 = 6.0_f64 * t12345 * t27503;
    let t27506 = t7962 * t1555;
    let t27508 = 4.0_f64 * t4189 * t27506;
    let t27509 = t2253 * t4310;
    let t27511 = 2.0_f64 * t4189 * t27509;
    let t27512 = t4244 * t573;
    (t27498, t27500, t27502, t27503, t27505, t27506, t27508, t27509, t27511, t27512)
}
