//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1269/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1269<F: Float>(t3123: F, t46175: F, t12065: F, t45323: F, t38997: F, t50123: F, t858: F, t866: F, t867: F, t39082: F, t1076: F, t2312: F, t29638: F, t3257: F, t3752: F, t3803: F, t38979: F, t38981: F, t46399: F, t50142: F, t904: F, t914: F, t916: F) -> (F, F, F, F, F, F) {
    let t50212 = t3123 * t46175 / F::cast_from(24.0_f64);
    let t50219 = F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t45323 * t12065;
    let t50220 = F::cast_from(35.0_f64) / F::cast_from(72.0_f64) * t38997;
    let t50230 = t866 * t867 * t858 * t50123 / F::cast_from(96.0_f64);
    let t50231 = F::cast_from(35.0_f64) / F::cast_from(12.0_f64) * t39082;
    let t50232 = F::cast_from(595.0_f64) / F::cast_from(576.0_f64) * t38979 - F::cast_from(119.0_f64) / F::cast_from(1152.0_f64) * t38981 - t50212 - t2312 * t3257 * t3803 * t3752 * t1076 / F::cast_from(16.0_f64) - t50219 - t50220 - t914 * t916 * t904 * t50142 / F::cast_from(512.0_f64) - F::cast_from(7.0_f64) / F::cast_from(96.0_f64) * t46399 + F::cast_from(595.0_f64) / F::cast_from(648.0_f64) * t29638 - t50230 + t50231;
    (t50212, t50219, t50220, t50230, t50231, t50232)
}
