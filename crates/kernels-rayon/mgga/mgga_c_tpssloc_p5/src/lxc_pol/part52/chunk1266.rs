//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1266/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1266(t25374: f64, t86716: f64, t193: f64, t200: f64, t8365: f64, t25: f64, t25353: f64, t606: f64, t7540: f64, t2752: f64, t32885: f64, t1877: f64, t2219: f64, t8370: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t118377 = t86716 * t25374;
    let t118381 = t193 * t200 * t8365;
    let t118387 = t25 * t25353;
    let t118393 = t606 * t7540;
    let t118399 = t32885 * t2752;
    let t118406 = t1877 * t8370 * t2219;
    (t118377, t118381, t118387, t118393, t118399, t118406)
}
