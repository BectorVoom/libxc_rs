//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 561/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk561(t44: f64, t51: f64, t2526: f64, t506: f64, t529: f64, t35: f64, t99: f64, t1216: f64, t415: f64, t903: f64, t101: f64, t419: f64, t906: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t45 = t44 <= zeta_threshold;
    let t52 = t51 <= zeta_threshold;
    let t2698 = t506 * t2526;
    let t2699 = t529 * t2698;
    let t2706 = t99 * t35;
    let t2710 = piecewise3(t45, 0.0_f64, 10.0_f64 / 9.0_f64 * t903 * t415 + 10.0_f64 / 3.0_f64 * t2706 * t1216);
    let t2713 = t101 * t35;
    let t2717 = piecewise3(t52, 0.0_f64, 10.0_f64 / 9.0_f64 * t906 * t419 - 10.0_f64 / 3.0_f64 * t2713 * t1216);
    let t2719 = t2710 / 2.0_f64 + t2717 / 2.0_f64;
    (t2698, t2699, t2706, t2713, t2719)
}
