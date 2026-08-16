//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1404/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1404(t23317: f64, t23384: f64, t225: f64, t23572: f64, t10348: f64, t1052: f64, t1066: f64, t11084: f64, t1922: f64, t1923: f64, t1955: f64, t23314: f64, t23346: f64, t23365: f64, t23369: f64, t23378: f64, t23395: f64, t23571: f64, t23582: f64, t23595: f64, t3016: f64, t3169: f64, t3174: f64, t3176: f64, t3206: f64, t349: f64, t388: f64, t6687: f64, t6699: f64, t6815: f64, t82803: f64, t83226: f64, t990: f64) -> f64 {
    let t83398 = t23384 * t23317;
    let t83408 = t23572 * t225;
    let t83417 = 2.0_f64 * t1052 * t3174 * t1955 * t11084 - 0.24674011002723396548e-1_f64 * t6687 * t3016 * t6699 - 0.3752886611772249944e0_f64 * t82803 * t1923 + 3.0_f64 * t990 * t23571 * t388 + 0.49348022005446793095e-1_f64 * t6687 * t23365 * t23395 + 6.0_f64 * t23369 * t3176 - 0.82246703342411321825e-2_f64 * t6687 * t10348 * t1922 - 0.82246703342411321826e-2_f64 * t83398 + 0.65797362673929057459e-1_f64 * t23346 * t23314 + 6.0_f64 * t3169 * t23378 + 6.0_f64 * t1052 * t3174 * t6815 * t3206 - 3.0_f64 * t83408 * t1066 - 0.29243272299524025538e-1_f64 * t23346 * t23595 + t349 * t83226 * t388 - 0.43864908449286038307e-1_f64 * t23346 * t23582;
    t83417
}
