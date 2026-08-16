//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 531/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk531(t108: f64, t2489: f64, t2496: f64, t109: f64, t95: f64, t1541: f64, t910: f64, t481: f64, t1212: f64, t889: f64, t35: f64, t472: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2498 = (t2489 + t2496) * t108;
    let t2504 = t109 * t95;
    let t2505 = t1541 * t910;
    let t2506 = t2505 * t481;
    let t2509 = t1212 * t889;
    let t2512 = t472 * t35;
    (t2498, t2504, t2505, t2506, t2509, t2512)
}
