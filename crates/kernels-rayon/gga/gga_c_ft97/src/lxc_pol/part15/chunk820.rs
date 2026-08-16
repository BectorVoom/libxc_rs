//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 820/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk820(t1701: f64, t3780: f64, t5295: f64, t1208: f64, t2035: f64, t5266: f64, t4064: f64, t10373: f64, t13643: f64, t13648: f64, t18032: f64, t18035: f64, t18038: f64, t18040: f64, t18081: f64, t21184: f64, t21188: f64, t21190: f64, t21194: f64, t21198: f64, t21202: f64, t21206: f64, t21213: f64, t21216: f64, t21218: f64, t21220: f64) -> (f64, f64, f64, f64) {
    let t22007 = t1701 * t3780 * t5295;
    let t22013 = t2035 * t5266 * t1208;
    let t22020 = t4064 * t5295;
    let t22059 = 0.48897200801234567904e0_f64 * t18081 + 0.16669500273148148149e-1_f64 * t18032 + 0.22226000364197530866e-1_f64 * t18035 - 0.33339000546296296299e-1_f64 * t18038 - 0.88904001456790123462e-1_f64 * t18040 + 0.51860667516460905352e-1_f64 * t21184 + 0.16669500273148148149e-1_f64 * t21188 + 0.26671200437037037038e0_f64 * t21190 - 0.13335600218518518519e0_f64 * t21194 + 0.66678001092592592595e-1_f64 * t21198 + 0.10001700163888888889e0_f64 * t21202 - 0.10001700163888888889e0_f64 * t21206 + 0.88904001456790123462e-1_f64 * t13643 + t10373 - 0.22818693707242798355e1_f64 * t21213 - 0.11113000182098765433e-1_f64 * t13648 + 0.48897200801234567904e0_f64 * t21216 - 0.13335600218518518519e0_f64 * t21218 - 0.17780800291358024692e0_f64 * t21220;
    (t22007, t22013, t22020, t22059)
}
