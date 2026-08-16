//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 801/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk801(t44: f64, t2452: f64, t406: f64, t2267: f64, t2625: f64, t2858: f64, t4904: f64, t889: f64, t1212: f64, t35: f64, t1216: f64, t472: f64, t1213: f64, t1219: f64, t2509: f64, t2512: f64, t40: f64, t6980: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t45 = t44 <= zeta_threshold;
    let t7054 = t406 * t2452;
    let t7055 = 8.0_f64 * t7054;
    let t7057 = t2858 * t2267 * t2625;
    let t7058 = 12.0_f64 * t7057;
    let t7059 = t4904 * t889;
    let t7062 = t1212 * t35;
    let t7067 = t472 * t1216;
    let t7072 = piecewise3(t45, 0.0_f64, 8.0_f64 / 27.0_f64 * t7059 * t1213 - 8.0_f64 / 9.0_f64 * t7062 * t6980 - 2.0_f64 / 9.0_f64 * t2509 * t1219 + 4.0_f64 / 3.0_f64 * t7067 - 4.0_f64 * t2512 * t40);
    (t7055, t7058, t7072)
}
