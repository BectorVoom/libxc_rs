//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2017/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2017(t1814: f64, t27105: f64, t81076: f64, t84481: f64, t90925: f64, t97023: f64, t97026: f64, t97030: f64, t97036: f64, t97040: f64, t97043: f64, t97046: f64, t97049: f64, t97055: f64, t97059: f64, t97063: f64, t97067: f64, t97070: f64) -> f64 {
    let t102614 = -t84481 + 0.52089578783527170489e-1_f64 * t81076 - 0.3289868133696452873e-1_f64 * t97023 + 0.16449340668482264365e-1_f64 * t97026 - 0.16449340668482264365e-1_f64 * t97030 - t90925 - 0.3289868133696452873e-1_f64 * t97036 - 0.3289868133696452873e-1_f64 * t97040 - 0.3289868133696452873e-1_f64 * t97043 + 0.9869604401089358619e-1_f64 * t97046 + 2.0_f64 * t1814 * t27105 - 0.16449340668482264365e-1_f64 * t97049 + 0.16449340668482264365e-1_f64 * t97055 - 0.9869604401089358619e-1_f64 * t97059 - 0.6579736267392905746e-1_f64 * t97063 - 0.6579736267392905746e-1_f64 * t97067 + 0.3289868133696452873e-1_f64 * t97070;
    t102614
}
