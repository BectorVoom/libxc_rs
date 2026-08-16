//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 781/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk781(t12037: f64, t12052: f64, t1974: f64, t1964: f64, t5396: f64, t755: f64, t5399: f64, t763: f64, t12019: f64, t10542: f64, t10559: f64, t10563: f64, t10566: f64, t10602: f64, t10707: f64, t10709: f64, t10712: f64, t10718: f64, t10752: f64, t10760: f64, t11999: f64, t12013: f64, t12018: f64, t12020: f64, t1966: f64, t5375: f64, t764: f64) -> f64 {
    let t12053 = t12037 + t12052;
    let t12054 = t12053 * t1974;
    let t12058 = 1.0_f64 / t5396 / t1964;
    let t12059 = t755 * t12058;
    let t12061 = 1.0_f64 / t5399 / t763;
    let t12062 = t12019 * t12061;
    let t12065 = -6.0_f64 * t11999 * t5375 + t10559 - t10563 - t10707 - t10709 - t10712 + t10718 - t10752 - t10760 + t10602 - 0.3109e-1_f64 * t12013 * t764 - 0.19298809906722418785e3_f64 * t12018 * t12020 + 1.0_f64 * t1966 * t12054 + 0.20691336878655965246e4_f64 * t12059 * t12062 - t10542 + t10566;
    t12065
}
