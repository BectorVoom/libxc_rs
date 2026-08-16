//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 967/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk967(t322: f64, t1338: f64, t3552: f64, t1142: f64, t6755: f64, t11216: f64, t1348: f64, t6767: f64, t11239: f64, t11244: f64, t1125: f64, t11273: f64, t11298: f64, t1307: f64, t2438: f64, t330: f64, t3517: f64, t352: f64, t3549: f64, t3556: f64, t6751: f64, t837: f64, t8481: f64, t855: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t323 = t322 <= 0.0_f64;
    let t331 = t322 <= 0.25e1_f64;
    let t332 = 0.25e1_f64 < t322;
    let t11302 = t1338 * t3552;
    let t11305 = t6755 * t1142;
    let t11310 = piecewise3(t332, t11216, 0.0_f64);
    let t11314 = t1348 * t3552;
    let t11319 = t6767 * t1142;
    let t11323 = piecewise5(t323, t1125 * t1307 * t330 + 2.0_f64 * t3517 * t837 * t330 + t11239 * t330 + t11244 * t330, t331, t11273 + t11298, -0.63e1_f64 * t3556 * t8481 - 0.42e1_f64 * t11302 * t2438 - 0.945e1_f64 * t11305 * t8481 - 0.21e1_f64 * t3549 * t6751 - 0.105e1_f64 * t855 * t11310 * t352 - 0.315e1_f64 * t11314 * t2438 - 0.1575e1_f64 * t3556 * t6751 - 0.23625e1_f64 * t11319 * t8481);
    (t11302, t11305, t11310, t11314, t11319, t11323)
}
