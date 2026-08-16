//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 679/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk679(t194: f64, t197: f64, t8991: f64, t815: f64, t9636: f64, t2740: f64, t375: f64, t89: f64, t10: f64, t296: f64, t3050: f64, t1636: f64, t825: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10355 = t8991 / t197 / t194;
    let t10362 = t815 * t815;
    let t10363 = 1.0_f64 / t10362;
    let t10373 = 0.18521666970164609055e-1_f64 * t9636;
    let t10394 = t89 * t375 * t2740;
    let t10397 = t10 * t3050 * t296;
    let t10398 = 14.0_f64 / 81.0_f64 * t10397;
    let t10400 = t89 * t1636 * t825;
    (t10355, t10363, t10373, t10394, t10397, t10398, t10400)
}
