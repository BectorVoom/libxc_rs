//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1023/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1023(t8227: f64, t256: f64, t4701: f64, t2112: f64, t4678: f64, t10708: f64, t13335: f64, t190: f64, t681: f64, t10698: f64, t1342: f64, t4741: f64, t725: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14147 = 0.10843581300301739842e-1_f64 * t8227;
    let t14151 = t256 * t4701;
    let t14156 = 4.0_f64 * t2112 * t4678;
    let t14157 = 8.0_f64 * t10708;
    let t14158 = t190 * t13335;
    let t14160 = 4.0_f64 * t681 * t14158;
    let t14162 = 8.0_f64 * t10698 * t1342;
    let t14163 = t4741 * t725;
    (t14147, t14151, t14156, t14157, t14160, t14162, t14163)
}
