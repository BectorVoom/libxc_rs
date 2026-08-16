//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 897/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk897(t3265: f64, t3315: f64, t3313: f64, t3236: f64, t3238: f64, t3245: f64, t3250: f64, t3254: f64, t1124: f64, t1128: f64) -> (f64, f64, f64, f64, f64) {
    let t3316 = t3265 * t3315;
    let t3318 = 0.16081979498692535067e2_f64 * t3313 * t3316;
    let t3319 = 0.22831111111111111111e-1_f64 * t3236;
    let t3324 = t3319 - 0.11415555555555555555e-1_f64 * t3238 - 0.11415555555555555555e-1_f64 * t3245 + 0.34246666666666666666e-1_f64 * t3250 + 0.17123333333333333333e-1_f64 * t3254;
    let t3327 = t1124 * t1128;
    (t3316, t3318, t3319, t3324, t3327)
}
