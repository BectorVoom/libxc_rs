//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 928/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk928(t206: f64, t8797: f64, t8912: f64, t209: f64, t880: f64, t208: f64, t214: f64, t2733: f64, t2742: f64, t2748: f64, t876: f64, t8769: f64, t8782: f64, t8785: f64, t8788: f64, t884: f64) -> (f64, f64) {
    let t210 = 0.0_f64 < t206;
    let t8913 = t8797 + t8912;
    let t8915 = piecewise3(t210, t8913, -t8913);
    let t8917 = t209 * t880 * t8915;
    let t8920 = -455.0_f64 / 1296.0_f64 * t8769 * t214 - 35.0_f64 / 144.0_f64 * t2733 * t884 - 7.0_f64 / 48.0_f64 * t876 * t2742 + 7.0_f64 / 96.0_f64 * t876 * t2748 - t208 * t8782 / 16.0_f64 + t8785 * t8788 / 16.0_f64 - t208 * t8917 / 96.0_f64;
    (t8913, t8920)
}
