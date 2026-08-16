//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2297/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2297(t23384: f64, t28684: f64, t1052: f64, t1065: f64, t17843: f64, t18074: f64, t1922: f64, t23346: f64, t25453: f64, t25743: f64, t28491: f64, t28500: f64, t28678: f64, t28681: f64, t3174: f64, t349: f64, t388: f64, t4552: f64, t4557: f64, t4660: f64, t6687: f64, t6776: f64, t7593: f64, t82469: f64, t83368: f64, t88937: f64, t88954: f64, t99859: f64) -> f64 {
    let t99864 = t23384 * t28684;
    let t99866 = t88937 + 2.0_f64 * t4552 * t7593 * t388 + 2.0_f64 * t1052 * t3174 * t28678 * t1065 + 0.36554090374405031923e-2_f64 * t6687 * t82469 * t28491 + 0.14621636149762012769e-1_f64 * t23346 * t28500 + 2.0_f64 * t18074 * t6776 - 0.82246703342411321825e-2_f64 * t6687 * t17843 * t1922 - t88954 + 0.18277045187202515961e-2_f64 * t83368 + 4.0_f64 * t4557 * t25453 + 4.0_f64 * t4660 * t25743 + t349 * t99859 * t388 + 0.43864908449286038307e-1_f64 * t23346 * t28681 - 0.54831135561607547883e-2_f64 * t99864;
    t99866
}
