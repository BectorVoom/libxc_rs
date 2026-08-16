//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1002/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1002(t322: f64, t1338: f64, t3774: f64, t12240: f64, t1348: f64, t11302: f64, t11305: f64, t11314: f64, t11319: f64, t11993: f64, t12267: f64, t12271: f64, t12273: f64, t12307: f64, t12338: f64, t2438: f64, t330: f64, t352: f64, t3549: f64, t3556: f64, t3675: f64, t3740: f64, t3742: f64, t837: f64, t838: f64, t855: f64, t9760: f64) -> (f64, f64, f64, f64) {
    let t323 = t322 <= 0.0_f64;
    let t331 = t322 <= 0.25e1_f64;
    let t332 = 0.25e1_f64 < t322;
    let t12348 = t1338 * t3774;
    let t12351 = piecewise3(t332, t12240, 0.0_f64);
    let t12355 = t1348 * t3774;
    let t12365 = piecewise5(t323, t330 * t3740 * t837 + t12267 * t330 + t12271 * t330 + t12273 * t330 + t3742 * t838, t331, t12307 + t12338, -0.63e1_f64 * t3556 * t11993 - 0.21e1_f64 * t11302 * t3675 - 0.945e1_f64 * t11305 * t11993 - 0.21e1_f64 * t3549 * t9760 - 0.21e1_f64 * t12348 * t2438 - 0.105e1_f64 * t855 * t12351 * t352 - 0.1575e1_f64 * t12355 * t2438 - 0.1575e1_f64 * t11314 * t3675 - 0.1575e1_f64 * t3556 * t9760 - 0.23625e1_f64 * t11319 * t11993);
    (t12348, t12351, t12355, t12365)
}
