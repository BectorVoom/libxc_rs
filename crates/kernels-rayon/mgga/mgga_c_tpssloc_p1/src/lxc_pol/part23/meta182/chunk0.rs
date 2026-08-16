//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 810/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk810(t261: f64, t2751: f64, t1053: f64, t68: f64, t134: f64, t976: f64, t271: f64, t2775: f64) -> (f64, f64, f64, f64, f64) {
    let t10143 = 1.0_f64 / t2751 / t261;
    let t10163 = t1053 * t1053;
    let t10164 = 1.0_f64 / t10163;
    let t10165 = t68 * t10164;
    let t10189 = t134 * t976;
    let t10213 = 1.0_f64 / t271 / t2775;
    (t10143, t10163, t10165, t10189, t10213)
}
