//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 807/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk807(t24829: f64, t462: f64, t7319: f64, t7327: f64, t7377: f64, t2144: f64, t3507: f64, t3625: f64, t1215: f64, t7348: f64, t1246: f64, t1170: f64, t7381: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t24830 = t462 * t24829;
    let t24833 = t7319 * t7327;
    let t24834 = t24833 * t7377;
    let t24837 = t2144 * t3507;
    let t24838 = t24837 * t3625;
    let t24840 = t7348 * t1215;
    let t24841 = t24840 * t1246;
    let t24844 = t1170 * t7381;
    (t24830, t24834, t24837, t24838, t24841, t24844)
}
