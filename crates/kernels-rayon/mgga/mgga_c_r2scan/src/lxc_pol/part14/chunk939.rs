//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 939/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk939(t10810: f64, t2150: f64, t574: f64, t3308: f64, t6402: f64, t1266: f64, t507: f64, t512: f64, t3332: f64, t6536: f64, t6535: f64, t6541: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10811 = t10810 * t2150;
    let t10812 = t574 * t10811;
    let t10813 = 0.23115257973478049502e0_f64 * t10812;
    let t10814 = t3308 * t6402;
    let t10815 = t574 * t10814;
    let t10818 = t512 * t1266 * t507;
    let t10820 = t3332 * t6536;
    let t10821 = t6535 * t10820;
    let t10823 = t3332 * t6541;
    (t10811, t10812, t10813, t10814, t10815, t10818, t10820, t10821, t10823)
}
