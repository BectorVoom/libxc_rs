//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1316/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1316(t1888: f64, t232: f64, t6646: f64, t87620: f64, t23110: f64, t23185: f64, t32822: f64, t112741: f64, t112743: f64, t112899: f64, t22986: f64, t25192: f64) -> (f64, f64, f64, f64, f64) {
    let t118764 = 0.16449340668482264365e-1_f64 * t1888 * t6646 * t87620 * t232;
    let t118766 = t23185 * t23110 * t32822;
    let t118767 = 0.82246703342411321825e-2_f64 * t118766;
    let t118791 = 0.82246703342411321825e-2_f64 * t112741;
    let t118792 = 0.76763589786250567036e-1_f64 * t112743;
    let t118802 = 0.3289868133696452873e-1_f64 * t22986 * t112899 * t25192;
    (t118764, t118767, t118791, t118792, t118802)
}
