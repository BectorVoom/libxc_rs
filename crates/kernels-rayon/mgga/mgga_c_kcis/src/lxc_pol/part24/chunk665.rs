//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 665/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk665(t1003: f64, t7709: f64, t5329: f64, t1014: f64, t2180: f64, t283: f64, t380: f64) -> (f64, f64, f64, f64, f64) {
    let t7710 = t7709 * t1003;
    let t7711 = t5329 * t7710;
    let t7716 = t1014 * t2180;
    let t7717 = 0.16581944444444444444e-2_f64 * t7716;
    let t7718 = t380 * t283;
    (t7710, t7711, t7716, t7717, t7718)
}
