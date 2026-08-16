//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 600/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk600(t10491: f64, t309: f64, t2360: f64, t870: f64, t2: f64, t7640: f64, t295: f64, t9567: f64, t303: f64, t3051: f64, t10478: f64, t305: f64, t631: f64, t7242: f64, t798: f64, t898: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10492 = t10491 * t309;
    let t10503 = t870 * t2360;
    let t10570 = t7640 * t2;
    let t10580 = t9567 * t295;
    let t10594 = 28.0_f64 / 27.0_f64 * t3051 * t303;
    let t10603 = t10491 * t2;
    let t10613 = t10478 * t2;
    let t10631 = 1.0_f64 / t305 / t631 / t898 / t798 / t7242 / 4.0_f64;
    (t10492, t10503, t10570, t10580, t10594, t10603, t10613, t10631)
}
