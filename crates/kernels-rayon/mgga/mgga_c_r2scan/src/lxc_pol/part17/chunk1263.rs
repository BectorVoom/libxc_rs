//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1263/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1263(t12918: f64, t1338: f64, t1348: f64, t10533: f64, t11302: f64, t11314: f64, t12348: f64, t12355: f64, t12683: f64, t2438: f64, t35220: f64, t3549: f64, t3556: f64, t3675: f64, t38958: f64, t38971: f64, t38976: f64, t42101: f64, t42121: f64, t42757: f64, t9760: f64) -> f64 {
    let t44855 = t1338 * t12918;
    let t44858 = t1348 * t12918;
    let t44873 = -0.354375e1_f64 * t38976 * t42757 - 0.42e1_f64 * t42121 * t3675 - 0.42e1_f64 * t12348 * t9760 - 0.945e1_f64 * t38958 * t12683 - 0.21e1_f64 * t11302 * t10533 - 0.21e1_f64 * t3549 * t35220 - 0.21e1_f64 * t44855 * t2438 - 0.1575e1_f64 * t44858 * t2438 - 0.315e1_f64 * t42101 * t3675 - 0.315e1_f64 * t12355 * t9760 - 0.1575e1_f64 * t11314 * t10533 - 0.1575e1_f64 * t3556 * t35220 - 0.23625e1_f64 * t38971 * t12683 - 0.63e1_f64 * t11314 * t12683;
    t44873
}
