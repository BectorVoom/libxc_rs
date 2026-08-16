//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 1035/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk1035(t13425: f64, t13459: f64, t858: f64, t225: f64, t4149: f64, t13050: f64, t13053: f64, t13059: f64, t13062: f64, t13065: f64, t13068: f64, t13072: f64, t13378: f64, t259: f64, t2597: f64, t2713: f64, t2720: f64, t4268: f64, t4273: f64, t4301: f64, t855: f64, t866: f64) -> f64 {
    let t13460 = t13425 + t13459;
    let t13461 = t858 * t13460;
    let t13463 = t4149 * t225;
    let t13470 = -6.0_f64 * t13050 * t855 - 2.0_f64 * t13053 * t866 + 2.0_f64 * t13059 * t855 + 2.0_f64 * t13062 * t259 - 2.0_f64 * t13065 * t866 + 2.0_f64 * t13068 * t259 + 4.0_f64 * t13072 * t855 + t13378 * t259 - t13461 * t855 - 2.0_f64 * t13463 * t866 + 4.0_f64 * t2597 * t4273 - 2.0_f64 * t2713 * t4301 + 2.0_f64 * t2720 * t4268;
    t13470
}
