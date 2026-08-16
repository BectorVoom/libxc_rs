//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 765/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk765(t10358: f64, t291: f64, t815: f64, t287: f64, t2726: f64, t820: f64, t2724: f64, t9636: f64, t9639: f64, t9642: f64, t9645: f64, t9648: f64, t9655: f64, t9660: f64, t9663: f64, t9668: f64, t9671: f64, t9675: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10359 = t291 * t10358;
    let t10362 = t815 * t815;
    let t10363 = 1.0_f64 / t10362;
    let t10364 = t287 * t10363;
    let t10365 = t2726 * t820;
    let t10369 = t2724 * t820;
    let t10373 = 0.18521666970164609055e-1_f64 * t9636;
    let t10384 = t10373 - 0.11113000182098765433e-1_f64 * t9639 + 0.22226000364197530866e-1_f64 * t9642 - 0.33339000546296296299e-1_f64 * t9645 + 0.16669500273148148149e-1_f64 * t9648 + 0.51860667516460905352e-1_f64 * t9655 - 0.13335600218518518519e0_f64 * t9660 + 0.66678001092592592595e-1_f64 * t9663 + 0.10001700163888888889e0_f64 * t9668 - 0.10001700163888888889e0_f64 * t9671 + 0.16669500273148148149e-1_f64 * t9675;
    (t10359, t10362, t10363, t10364, t10365, t10369, t10384)
}
