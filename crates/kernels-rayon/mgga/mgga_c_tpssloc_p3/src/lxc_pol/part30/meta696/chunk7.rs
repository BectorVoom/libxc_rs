//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2234/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2234(t1888: f64, t232: f64, t5631: f64, t6646: f64, t828: f64, t25319: f64, t4119: f64, t6552: f64, t6637: f64, t16935: f64, t17034: f64, t25261: f64, t25281: f64, t4162: f64, t4281: f64, t5575: f64, t6660: f64, t7535: f64, t81689: f64, t81717: f64, t82011: f64, t87604: f64, t87613: f64, t87619: f64, t87635: f64, t87669: f64, t87680: f64, t92781: f64, t92794: f64) -> f64 {
    let t98571 = t1888 * t6646 * t5631 * t828 * t232;
    let t98575 = t6552 * t6637 * t25319 * t4119;
    let t98587 = t87604 - 0.82246703342411321825e-2_f64 * t98571 - t81689 - 0.3289868133696452873e-1_f64 * t98575 - t87613 + t87619 - 0.25587863262083522345e0_f64 * t87635 + 4.0_f64 * t4281 * t25261 * t16935 - t92781 + t81717 + 4.0_f64 * t17034 * t25281 - t92794 + t87669 + t87680 + t5575 * t6660 - 0.63969658155208805863e-1_f64 * t82011 + 2.0_f64 * t4162 * t7535;
    t98587
}
