//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 996/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk996(t12069: f64, t12089: f64, t12107: f64, t12225: f64, t354: f64, t11866: f64, t11876: f64, t11886: f64, t11035: f64, t11046: f64, t11052: f64, t11206: f64, t11215: f64, t11868: f64, t11870: f64, t11872: f64, t11874: f64, t11878: f64, t11883: f64, t11889: f64) -> (f64, f64, f64) {
    let t12227 = t12069 + t12089 + t12107 + t12225;
    let t12228 = t354 * t12227;
    let t12230 = 2.0_f64 / 3.0_f64 * t11866;
    let t12235 = 2.0_f64 / 3.0_f64 * t11876;
    let t12238 = 4.0_f64 / 3.0_f64 * t11886;
    let t12240 = -t11206 - t11035 - t12230 - t11868 / 2.0_f64 + t11870 / 4.0_f64 - t11872 / 4.0_f64 + t11874 / 2.0_f64 + t12235 + t11878 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t11883 - t12238 + t11889 / 2.0_f64 + t11046 - t11052 - t11215;
    (t12227, t12228, t12240)
}
