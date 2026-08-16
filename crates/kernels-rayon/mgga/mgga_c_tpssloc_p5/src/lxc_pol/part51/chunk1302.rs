//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1302/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1302(t22986: f64, t32814: f64, t82159: f64, t32815: f64, t81591: f64, t112899: f64, t1888: f64, t25045: f64, t23270: f64, t30633: f64, t98960: f64, t25038: f64, t25040: f64) -> (f64, f64, f64, f64, f64) {
    let t118479 = 0.3289868133696452873e-1_f64 * t22986 * t82159 * t32814;
    let t118480 = t81591 * t32815;
    let t118481 = 0.76763589786250567037e-1_f64 * t118480;
    let t118484 = 0.3289868133696452873e-1_f64 * t1888 * t112899 * t25045;
    let t118488 = 0.6579736267392905746e-1_f64 * t22986 * t23270 * t30633 * t98960;
    let t118491 = 0.9869604401089358619e-1_f64 * t25038 * t112899 * t25040;
    (t118479, t118481, t118484, t118488, t118491)
}
