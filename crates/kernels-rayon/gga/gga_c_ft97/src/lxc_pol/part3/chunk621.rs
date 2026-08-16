//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 621/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk621(t291: f64, t5266: f64, t1208: f64, t3780: f64, t1701: f64, t2725: f64, t2730: f64, t3796: f64, t3804: f64, t5031: f64, t5034: f64, t5039: f64, t5043: f64, t5047: f64) -> (f64, f64, f64, f64, f64) {
    let t5267 = t5266 * t291;
    let t5272 = t3780 * t1208;
    let t5273 = t1701 * t5272;
    let t5284 = t1208 * t1208;
    let t5285 = t2725 * t5284;
    let t5295 = 0.48897200801234567903e0_f64 * t5031 - 0.88904001456790123461e-1_f64 * t3796 - 0.88904001456790123461e-1_f64 * t5034 - t2730 + 0.11113000182098765433e-1_f64 * t3804 + 0.22226000364197530865e-1_f64 * t5039 - 0.33339000546296296298e-1_f64 * t5043 + 0.16669500273148148149e-1_f64 * t5047;
    (t5267, t5273, t5284, t5285, t5295)
}
