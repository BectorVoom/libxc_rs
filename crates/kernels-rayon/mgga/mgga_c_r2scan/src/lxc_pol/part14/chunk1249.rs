//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1249/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1249(t12351: f64, t1348: f64, t3774: f64, t6767: f64, t1338: f64, t6755: f64, t11302: f64, t11314: f64, t11993: f64, t12348: f64, t12355: f64, t2438: f64, t31929: f64, t3549: f64, t3556: f64, t3675: f64, t38953: f64, t38966: f64, t6751: f64, t8481: f64, t9760: f64) -> f64 {
    let t42101 = t1348 * t12351;
    let t42106 = t6767 * t3774;
    let t42121 = t1338 * t12351;
    let t42128 = t6755 * t3774;
    let t42131 = -0.126e2_f64 * t11314 * t11993 - 0.315e1_f64 * t42101 * t2438 - 0.1575e1_f64 * t12355 * t6751 - 0.23625e1_f64 * t42106 * t8481 - 0.1575e1_f64 * t38966 * t3675 - 0.315e1_f64 * t11314 * t9760 - 0.1575e1_f64 * t3556 * t31929 - 0.21e1_f64 * t38953 * t3675 - 0.42e1_f64 * t11302 * t9760 - 0.21e1_f64 * t3549 * t31929 - 0.42e1_f64 * t42121 * t2438 - 0.21e1_f64 * t12348 * t6751 - 0.63e1_f64 * t12355 * t8481 - 0.945e1_f64 * t42128 * t8481;
    t42131
}
