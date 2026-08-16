//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 963/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk963(t12032: f64, t2902: f64, t14295: f64, t4342: f64, t12148: f64, t2798: f64, t1016: f64, t39340: f64, t1382: f64, t4349: f64, t605: f64, t1022: f64, t3720: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t49970 = 2.0_f64 * t12032 * t2902;
    let t49972 = 4.0_f64 * t4342 * t14295;
    let t49974 = 2.0_f64 * t2798 * t12148;
    let t49977 = 2.0_f64 * t39340 * t1016;
    let t49980 = 4.0_f64 * t1382 * t1016 * t12148;
    let t49983 = 12.0_f64 * t4349 * t14295 * t605;
    let t49989 = t1022 * t3720;
    (t49970, t49972, t49974, t49977, t49980, t49983, t49989)
}
