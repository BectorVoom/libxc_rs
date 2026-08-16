//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 816/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk816(t51: f64, t101: f64, t1216: f64, t1225: f64, t1228: f64, t2517: f64, t2520: f64, t2713: f64, t40: f64, t6995: f64, t906: f64, t7276: f64, t552: f64, zeta_threshold: f64) -> (f64, f64) {
    let t52 = t51 <= zeta_threshold;
    let t7288 = piecewise3(t52, 0.0_f64, -10.0_f64 / 27.0_f64 * t2517 * t1225 - 40.0_f64 / 9.0_f64 * t2520 * t6995 + 10.0_f64 / 9.0_f64 * t906 * t1228 - 10.0_f64 / 3.0_f64 * t101 * t1216 + 10.0_f64 * t2713 * t40);
    let t7290 = t7276 / 2.0_f64 + t7288 / 2.0_f64;
    let t7291 = t552 * t7290;
    (t7290, t7291)
}
