//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2085/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2085(t2: f64, t870: f64, t584: f64, t776: f64, t22959: f64, t1408: f64, t2553: f64, t10143: f64, t606: f64, t25374: f64, t1877: f64, t1915: f64) -> (f64, f64, f64, f64) {
    let t86753 = t870 * t2;
    let t86755 = t86753 * t584 * t776;
    let t86757 = 6.0_f64 * t22959 * t86755;
    let t86764 = t1408 * t2553;
    let t86770 = t10143 * t606;
    let t86771 = t86770 * t25374;
    let t86775 = t1877 * t1915 * t584;
    (t86757, t86764, t86771, t86775)
}
