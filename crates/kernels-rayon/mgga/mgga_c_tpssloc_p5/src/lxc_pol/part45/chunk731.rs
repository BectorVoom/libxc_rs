//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 731/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk731(t22893: f64, t6969: f64, t22892: f64, t3787: f64, t6604: f64, t22740: f64, t3792: f64, t1992: f64, t1336: f64, t2013: f64, t22743: f64, t22746: f64, t22749: f64, t22753: f64, t22871: f64, t22874: f64, t22877: f64, t22879: f64, t22884: f64, t22888: f64, t3773: f64, t544: f64) -> (f64, f64, f64, f64) {
    let t22894 = t22893 * t6969;
    let t22895 = t22892 * t22894;
    let t22896 = 0.16449340668482264365e-1_f64 * t22895;
    let t22897 = t6604 * t3787;
    let t22898 = t22740 * t3792;
    let t22899 = t22897 * t22898;
    let t22900 = t1992 * t22899;
    let t22903 = -0.82246703342411321825e-2_f64 * t22743 + t22746 + 0.49348022005446793095e-1_f64 * t22749 + t22753 + t544 * t22871 - 2.0_f64 * t1336 * t22874 - t1336 * t22877 - t1336 * t22879 - 0.3289868133696452873e-1_f64 * t22884 - 0.16449340668482264365e-1_f64 * t22888 + t22896 + 0.16449340668482264365e-1_f64 * t22900 + t3773 * t2013;
    (t22895, t22897, t22900, t22903)
}
