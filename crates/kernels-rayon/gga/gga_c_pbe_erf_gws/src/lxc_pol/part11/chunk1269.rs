//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1269/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1269(t3123: f64, t46175: f64, t12065: f64, t45323: f64, t38997: f64, t50123: f64, t858: f64, t866: f64, t867: f64, t39082: f64, t1076: f64, t2312: f64, t29638: f64, t3257: f64, t3752: f64, t3803: f64, t38979: f64, t38981: f64, t46399: f64, t50142: f64, t904: f64, t914: f64, t916: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t50212 = t3123 * t46175 / 24.0_f64;
    let t50219 = 7.0_f64 / 48.0_f64 * t45323 * t12065;
    let t50220 = 35.0_f64 / 72.0_f64 * t38997;
    let t50230 = t866 * t867 * t858 * t50123 / 96.0_f64;
    let t50231 = 35.0_f64 / 12.0_f64 * t39082;
    let t50232 = 595.0_f64 / 576.0_f64 * t38979 - 119.0_f64 / 1152.0_f64 * t38981 - t50212 - t2312 * t3257 * t3803 * t3752 * t1076 / 16.0_f64 - t50219 - t50220 - t914 * t916 * t904 * t50142 / 512.0_f64 - 7.0_f64 / 96.0_f64 * t46399 + 595.0_f64 / 648.0_f64 * t29638 - t50230 + t50231;
    (t50212, t50219, t50220, t50230, t50231, t50232)
}
