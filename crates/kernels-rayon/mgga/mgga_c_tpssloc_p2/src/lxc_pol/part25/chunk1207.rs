//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1207/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1207(t81920: f64, t81954: f64, t81907: f64, t81909: f64, t81912: f64, t81918: f64, t81924: f64, t81926: f64, t81928: f64, t81930: f64, t81934: f64, t81936: f64, t81940: f64, t81943: f64, t81946: f64, t81949: f64, t81957: f64, t81960: f64, t81964: f64, t81972: f64) -> f64 {
    let t84921 = 595.0_f64 / 2592.0_f64 * t81920;
    let t84932 = 0.67287926823567318088e-4_f64 * t81954;
    let t84937 = 0.24223653656484234512e-2_f64 * t81907 + 0.84782787797694820791e-2_f64 * t81909 - 0.67826230238155856633e-1_f64 * t81912 - 0.40372756094140390854e-3_f64 * t81918 - t84921 + 0.20186378047070195427e-3_f64 * t81924 - 7.0_f64 / 384.0_f64 * t81926 + 119.0_f64 / 1152.0_f64 * t81928 - t81930 / 24.0_f64 - 0.4069573814289351398e0_f64 * t81934 + 0.50869672678616892474e-1_f64 * t81936 - 0.24223653656484234512e-2_f64 * t81940 - 35.0_f64 / 36.0_f64 * t81943 + 3.0_f64 / 8.0_f64 * t81946 + 0.50869672678616892474e-1_f64 * t81949 - t84932 - 7.0_f64 / 8.0_f64 * t81957 - t81960 / 2.0_f64 - 0.35608770875031824732e0_f64 * t81964 - 0.13565246047631171326e0_f64 * t81972;
    t84937
}
