//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 569/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk569(t2128: f64, t1254: f64, t2119: f64, t4037: f64, t4041: f64, t6020: f64, t7914: f64, t7917: f64, t7920: f64, t1235: f64, t4054: f64, t1242: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7927 = t2128 * t2128;
    let t7928 = t7927 * t1254;
    let t7931 = t2119 * t2119;
    let t7932 = t4037 * t7931;
    let t7938 = t4041 + 2.0_f64 / 9.0_f64 * t6020 - 2.0_f64 / 9.0_f64 * t7914 + 2.0_f64 / 3.0_f64 * t7917 - t7920 / 3.0_f64;
    let t7939 = t1235 * t7938;
    let t7945 = t4054 * t7931;
    let t7947 = t1242 * t7938;
    (t7927, t7928, t7931, t7932, t7938, t7939, t7945, t7947)
}
