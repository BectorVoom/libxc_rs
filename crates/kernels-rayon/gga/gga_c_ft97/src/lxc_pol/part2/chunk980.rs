//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 980/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk980(t1240: f64, t2770: f64, t2877: f64, t848: f64, t2884: f64, t15143: f64, t15147: f64, t15150: f64, t15154: f64, t15159: f64, t15164: f64, t15168: f64, t15170: f64, t15172: f64, t15177: f64, t15180: f64, t15185: f64, t15190: f64, t1901: f64, t193: f64, t446: f64, t89: f64) -> f64 {
    let t15191 = t2770 * t1240;
    let t15192 = t15191 * t2877;
    let t15195 = t848 * t1240;
    let t15196 = t15195 * t2884;
    let t15199 = t89 * t193 * t15143 / 3.0_f64 - 4.0_f64 / 27.0_f64 * t15147 - t446 * t15150 / 9.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t15154 + 2.0_f64 / 3.0_f64 * t446 * t15159 + t446 * t15164 / 3.0_f64 - t15168 - t15170 + 4.0_f64 / 3.0_f64 * t446 * t15172 + 4.0_f64 / 3.0_f64 * t446 * t15177 - 22.0_f64 / 27.0_f64 * t15180 - 2.0_f64 / 9.0_f64 * t1901 * t15185 - t15190 + 2.0_f64 / 9.0_f64 * t1901 * t15192 + 2.0_f64 / 9.0_f64 * t1901 * t15196;
    t15199
}
