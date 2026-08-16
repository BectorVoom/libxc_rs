//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1254/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1254(t12089: f64, t12091: f64, t12044: f64, t12048: f64, t12057: f64, t12059: f64, t12087: f64, t12094: f64, t15904: f64, t15910: f64, t15911: f64, t15915: f64, t15916: f64, t15917: f64, t15923: f64, t3734: f64, t3918: f64, t3919: f64, t5122: f64, t5126: f64, t5161: f64, t5187: f64, t5308: f64, t9789: f64, t9793: f64) -> (f64, f64, f64) {
    let t15927 = 0.5848223622634646207e0_f64 * t12089;
    let t15928 = 0.34631718211362927518e2_f64 * t12091;
    let t15929 = -6.0_f64 * t15904 * t3918 * t5161 + 6.0_f64 * t3734 * t5122 * t5126 + 6.0_f64 * t3918 * t3919 * t5187 + 12.0_f64 * t3919 * t5126 * t5308 - t12044 - t12048 - t12057 - t12059 + t12087 - t12094 + t15910 + t15911 - t15915 - t15916 + t15917 + t15923 - t15927 - t15928 - t9789 + t9793;
    (t15927, t15928, t15929)
}
