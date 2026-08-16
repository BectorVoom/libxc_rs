//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1148/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1148(t14911: f64, t4283: f64, t3931: f64, t4212: f64, t4216: f64, t140: f64, t5206: f64, t1098: f64, t3054: f64, t4245: f64, t4231: f64, t4246: f64) -> (f64, f64, f64, f64, f64) {
    let t15827 = t4283 * t14911;
    let t15828 = t3931 * t15827;
    let t15832 = t4212 * t4216;
    let t15834 = t140 * t5206;
    let t15835 = t1098 * t15834;
    let t15837 = t3054 * t4245;
    let t15838 = t4231 * t15837;
    let t15839 = t3931 * t15838;
    let t15842 = t4231 * t4246;
    (t15828, t15832, t15835, t15839, t15842)
}
