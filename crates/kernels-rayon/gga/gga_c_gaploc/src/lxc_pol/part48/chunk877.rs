//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 877/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk877(t3431: f64, t8469: f64, t13556: f64, t7129: f64, t35709: f64, t935: f64, t2508: f64, t2580: f64, t11595: f64, t2586: f64, t13486: f64, t7137: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t44967 = t8469 * t3431;
    let t44972 = 0.15381052460284448567e-1_f64 * t7129 * t13556;
    let t44973 = t35709 * t935;
    let t44976 = 0.15381052460284448567e-1_f64 * t2508 * t2580 * t44973;
    let t44990 = 0.23071578690426672851e-1_f64 * t2508 * t11595 * t2586;
    let t44992 = 0.30762104920568897135e-1_f64 * t7137 * t13486;
    (t44967, t44972, t44973, t44976, t44990, t44992)
}
