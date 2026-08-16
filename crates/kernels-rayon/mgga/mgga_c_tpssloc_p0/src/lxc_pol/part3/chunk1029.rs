//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 1029/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk1029(t10012: f64, t10014: f64, t10026: f64, t10029: f64, t10030: f64, t10036: f64, t10038: f64, t13333: f64, t13337: f64, t13345: f64, t13347: f64, t13353: f64, t13359: f64, t13362: f64, t13365: f64, t13368: f64, t1516: f64, t249: f64, t2623: f64, t2643: f64, t2703: f64, t2707: f64, t4172: f64, t4178: f64, t4261: f64, t843: f64, t849: f64, t9990: f64) -> f64 {
    let t13375 = t4178 * t13333 / 512.0_f64 + t13337 * t249 / 3072.0_f64 - t9990 * t1516 / 768.0_f64 - t2623 * t4261 / 384.0_f64 + t13345 - t843 * t13347 / 768.0_f64 - 5.0_f64 / 384.0_f64 * t2643 * t13353 - 7.0_f64 / 4608.0_f64 * t10012 + 119.0_f64 / 6912.0_f64 * t10014 - t10026 - t10029 + t13359 + t13362 - t4172 * t2707 / 768.0_f64 - t13365 * t849 / 384.0_f64 - 119.0_f64 / 3456.0_f64 * t13368 + 5.0_f64 / 768.0_f64 * t4172 * t2703 - 7.0_f64 / 48.0_f64 * t10030 - 35.0_f64 / 108.0_f64 * t10036 + 7.0_f64 / 144.0_f64 * t10038;
    t13375
}
