//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 813/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk813(t481: f64, t7184: f64, t1234: f64, t2505: f64, t490: f64, t7088: f64, t109: f64, t111: f64, t1536: f64, t1544: f64, t1547: f64, t2498: f64, t2504: f64, t2506: f64, t2527: f64, t486: f64, t491: f64, t7165: f64, t7175: f64, t7181: f64, t915: f64, t917: f64) -> f64 {
    let t7185 = t7184 * t481;
    let t7188 = t2505 * t1234;
    let t7191 = t490 * t7088;
    let t7194 = 3.0_f64 * t109 * t7191 - t7165 * t111 + 3.0_f64 * t1536 * t917 - 12.0_f64 * t915 * t1544 + 3.0_f64 * t915 * t1547 + 6.0_f64 * t2498 * t491 + 60.0_f64 * t2504 * t7181 - 24.0_f64 * t2504 * t7185 - 12.0_f64 * t2504 * t7188 - 24.0_f64 * t7175 * t2506 + 6.0_f64 * t486 * t2527;
    t7194
}
