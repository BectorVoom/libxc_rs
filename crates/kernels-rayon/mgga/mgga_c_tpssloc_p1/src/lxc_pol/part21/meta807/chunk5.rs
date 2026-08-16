//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2815/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2815(t17095: f64, t225: f64, t17098: f64, t10049: f64, t13042: f64, t13059: f64, t13463: f64, t1528: f64, t17052: f64, t17064: f64, t17090: f64, t17092: f64, t218: f64, t252: f64, t259: f64, t2710: f64, t2713: f64, t2718: f64, t2719: f64, t2720: f64, t2742: f64, t2743: f64, t40890: f64, t4268: f64, t4273: f64, t4301: f64, t46508: f64, t5558: f64, t5636: f64, t5637: f64, t5657: f64, t59229: f64, t59328: f64, t855: f64, t866: f64) -> f64 {
    let t59519 = t17095 * t225;
    let t59537 = t17098 * t225;
    let t59558 = -4.0_f64 * t59519 * t866 + t5558 * t2710 * t259 + 2.0_f64 * t10049 * t5637 - 2.0_f64 * t17092 * t2743 - t17052 * t2743 - 12.0_f64 * t2713 * t17064 + t218 * t59328 * t259 - 2.0_f64 * t46508 * t1528 + 2.0_f64 * t17052 * t2720 - 2.0_f64 * t59537 * t866 + 2.0_f64 * t855 * t2718 * t5657 * t2742 + 8.0_f64 * t13042 * t4273 - 4.0_f64 * t13463 * t4301 + 4.0_f64 * t4268 * t13059 + 24.0_f64 * t855 * t40890 * t5636 * t2719 + 2.0_f64 * t17090 * t2720 + t59229 * t252 * t259;
    t59558
}
