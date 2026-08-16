//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 940/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk940(t10823: f64, t2147: f64, t3332: f64, t6166: f64, t6165: f64, t3333: f64, t6395: f64, t1266: f64, t260: f64, t259: f64, t277: f64, t254: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10824 = t2147 * t10823;
    let t10826 = t3332 * t6166;
    let t10827 = t6165 * t10826;
    let t10829 = t6395 * t3333;
    let t10831 = t260 * t1266;
    let t10833 = t259 * t10831 * t277;
    let t10834 = t254 * t10833;
    (t10824, t10826, t10827, t10829, t10831, t10833, t10834)
}
