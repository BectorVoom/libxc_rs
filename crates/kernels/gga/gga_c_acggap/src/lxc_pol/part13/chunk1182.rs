//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1182/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1182<F: Float>(t36175: F, t30689: F, t5286: F, t1165: F, t2068: F, t20972: F, t7351: F, t31759: F, t31761: F, t31763: F, t31774: F, t31782: F, t31790: F, t36147: F, t36149: F, t36152: F, t36157: F, t36160: F, t36163: F, t36165: F, t36169: F, t36173: F) -> F {
    let t36176 = F::cast_from(0.94344276868812456204e-3_f64) * t36175;
    let t36177 = t30689 * t5286;
    let t36178 = F::cast_from(0.34299214494455789578e-2_f64) * t36177;
    let t36181 = t2068 * t1165 * t7351 * t20972;
    let t36183 = -F::cast_from(0.3572834843172478081e-3_f64) * t31759 - F::cast_from(0.42874018118069736972e-3_f64) * t31761 - F::cast_from(0.21437009059034868486e-3_f64) * t31763 + t36147 / F::cast_from(16.0_f64) + t36149 / F::cast_from(48.0_f64) + t36152 + F::cast_from(0.16809375e0_f64) * t31774 + F::cast_from(0.84046875e-1_f64) * t31782 - F::cast_from(0.5603125e-1_f64) * t31790 - t36157 - F::cast_from(0.31448092289604152068e-3_f64) * t36160 + t36163 + F::cast_from(0.42874018118069736972e-3_f64) * t36165 + F::cast_from(0.42874018118069736972e-3_f64) * t36169 + F::cast_from(0.21437009059034868486e-3_f64) * t36173 - t36176 - t36178 - F::cast_from(0.94344276868812456204e-3_f64) * t36181;
    t36183
}
