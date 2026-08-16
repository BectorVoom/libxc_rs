//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 710/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk710(t2987: f64, t984: f64, t343: f64, t883: f64, t607: f64, t2775: f64, t344: f64, t2822: f64, t225: f64, t991: f64, t1008: f64, t191: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2988 = t2987 * t984;
    let t2989 = t343 * t883;
    let t2990 = t2989 * t607;
    let t2994 = t344 * t2775;
    let t3003 = 5.0_f64 / 18.0_f64 * t2822;
    let t3026 = t991 * t225;
    let t3030 = 1.0_f64 / t1008 / t191;
    (t2988, t2989, t2990, t2994, t3003, t3026, t3030)
}
