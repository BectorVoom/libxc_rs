//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 957/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk957(t245: f64, t18759: f64, t18772: f64, t1178: f64, t18: f64, t15625: f64, t1577: f64, t21: f64, t267: f64, t363: f64, t4011: f64, t4431: f64, t5: f64, t5186: f64, t776: f64, t920: f64) -> f64 {
    let t246 = 10000000.0_f64 <= t245;
    let t18773 = t18759 + t18772;
    let t18783 = t1178 * t18;
    let t18793 = piecewise3(t246, 0.0_f64, t5 * t18773 * t21 / 4.0_f64 + t5 * t5186 * t363 / 4.0_f64 + t5 * t4011 * t920 / 2.0_f64 + t5 * t18783 * t1577 + t5 * t776 * t4431 / 4.0_f64 + t5 * t267 * t15625 / 4.0_f64);
    t18793
}
