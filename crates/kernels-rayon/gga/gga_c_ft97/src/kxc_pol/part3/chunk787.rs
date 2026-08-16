//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 787/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk787(t1825: f64, t4589: f64, t83: f64, t11988: f64, t16150: f64, t11987: f64, t16169: f64, t3194: f64, t3193: f64, t11567: f64, t11578: f64, t11593: f64, t11610: f64, t11612: f64, t11632: f64, t11821: f64, t11826: f64, t16200: f64, t16205: f64, t16210: f64, t16213: f64, t1901: f64, t446: f64, t8233: f64) -> (f64, f64) {
    let t16215 = t1825 * t4589;
    let t16216 = t83 * t16215;
    let t16219 = t11988 * t16150;
    let t16220 = t11987 * t16219;
    let t16223 = t3194 * t16169;
    let t16224 = t3193 * t16223;
    let t16227 = -t11567 + 8.0_f64 / 27.0_f64 * t11578 - 2.0_f64 * t446 * t16200 + 4.0_f64 / 3.0_f64 * t446 * t16205 - 4.0_f64 / 81.0_f64 * t8233 + 2.0_f64 / 3.0_f64 * t446 * t16210 + t16213 / 9.0_f64 + t11610 - t11612 - t11632 - t446 * t16216 / 3.0_f64 + t11821 - t11826 - 10.0_f64 / 81.0_f64 * t1901 * t16220 - 8.0_f64 / 27.0_f64 * t11593 * t16224;
    (t16215, t16227)
}
