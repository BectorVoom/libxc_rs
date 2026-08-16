//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 745/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk745(t1814: f64, t3293: f64, t1824: f64, t4684: f64, t5093: f64, t10593: f64, t1842: f64, t1856: f64, t10585: f64, t1659: f64, t3845: f64, t429: f64, t686: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11506 = t1814 * t3293;
    let t11507 = t11506 * t1824;
    let t11510 = t5093 * t4684;
    let t11513 = t1842 * t10593;
    let t11516 = t1856 * t10593;
    let t11519 = t1659 * t10585;
    let t11524 = 0.27323333333333333333e-1_f64 * t429 * t3845 * t686;
    (t11507, t11510, t11513, t11516, t11519, t11524)
}
