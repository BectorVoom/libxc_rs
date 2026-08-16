//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 1075/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk1075(t2139: f64, t27: f64, t3118: f64, t558: f64, t36634: f64, t40972: f64, t40975: f64, t7192: f64, t16156: f64, t9194: f64, t9190: f64, t1001: f64, t236: f64, t3351: f64, t35312: f64, t551: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t42196 = t2139 * t27 * t3118 * t558;
    let t42199 = t36634 * t40972;
    let t42201 = t7192 * t40975;
    let t42204 = t16156 * t9194;
    let t42205 = 0.17877131955185092547e-3_f64 * t42204;
    let t42206 = t16156 * t9190;
    let t42207 = 0.11918087970123395031e-3_f64 * t42206;
    let t42211 = t3351 * t35312 * t236 * t551 * t1001;
    (t42196, t42199, t42201, t42205, t42207, t42211)
}
