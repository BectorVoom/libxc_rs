//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 391/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk391(t337: f64, t39: f64, t1887: f64, t60: f64, t976: f64, t343: f64, t883: f64, t2775: f64, t344: f64, t2822: f64, t1008: f64, t191: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2985 = t39 * t337;
    let t2986 = t2985 * t1887;
    let t2987 = t60 * t976;
    let t2989 = t343 * t883;
    let t2994 = t344 * t2775;
    let t3003 = 5.0_f64 / 18.0_f64 * t2822;
    let t3030 = 1.0_f64 / t1008 / t191;
    (t2986, t2987, t2989, t2994, t3003, t3030)
}
