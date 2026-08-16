//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 933/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk933(t10810: f64, t2150: f64, t574: f64, t1266: f64, t507: f64, t512: f64, t260: f64, t259: f64, t277: f64, t254: f64, t3316: f64, t776: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10811 = t10810 * t2150;
    let t10812 = t574 * t10811;
    let t10813 = 0.23115257973478049502e0_f64 * t10812;
    let t10818 = t512 * t1266 * t507;
    let t10831 = t260 * t1266;
    let t10833 = t259 * t10831 * t277;
    let t10834 = t254 * t10833;
    let t10839 = t776 * t3316;
    (t10811, t10813, t10818, t10831, t10833, t10834, t10839)
}
