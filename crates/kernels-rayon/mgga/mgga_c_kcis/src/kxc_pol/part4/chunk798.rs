//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 798/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk798(t278: f64, t417: f64, t4967: f64, t4768: f64, t1001: f64, t286: f64, t1700: f64, t1706: f64, t285: f64, t2870: f64, t2872: f64, t2879: f64, t2882: f64, t2885: f64, t4937: f64, t4940: f64, t4944: f64, t4948: f64, t4953: f64, t4959: f64, t4963: f64, t984: f64, t991: f64) -> (f64, f64, f64, f64, f64) {
    let t288 = 0.0_f64 < t278;
    let t4968 = t417 * t4967;
    let t4972 = piecewise3(t288, t4768, -t4768);
    let t4973 = t1001 * t4972;
    let t4974 = t286 * t4973;
    let t4977 = -t2870 / 108.0_f64 - t2879 + t2882 / 864.0_f64 - t2885 / 288.0_f64 - t2872 * t1700 / 108.0_f64 + t4937 / 864.0_f64 + t991 * t4940 / 216.0_f64 - t991 * t4944 / 288.0_f64 - t991 * t4948 / 144.0_f64 + t991 * t4953 / 144.0_f64 + t984 * t1706 / 36.0_f64 - t4959 / 288.0_f64 - t991 * t4963 / 288.0_f64 + t991 * t4968 / 48.0_f64 - t285 * t4974 / 96.0_f64;
    (t4968, t4972, t4973, t4974, t4977)
}
