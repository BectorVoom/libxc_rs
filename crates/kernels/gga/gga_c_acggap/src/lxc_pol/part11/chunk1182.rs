//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1182/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1182<F: Float>(t36175: F, t30689: F, t5286: F, t1165: F, t2068: F, t20972: F, t7351: F, t31759: F, t31761: F, t31763: F, t31774: F, t31782: F, t31790: F, t36147: F, t36149: F, t36152: F, t36157: F, t36160: F, t36163: F, t36165: F, t36169: F, t36173: F) -> F {
    let t36176 = F::new(0.94344276868812456204e-3) * t36175;
    let t36177 = t30689 * t5286;
    let t36178 = F::new(0.34299214494455789578e-2) * t36177;
    let t36181 = t2068 * t1165 * t7351 * t20972;
    let t36183 = -F::new(0.3572834843172478081e-3) * t31759 - F::new(0.42874018118069736972e-3) * t31761 - F::new(0.21437009059034868486e-3) * t31763 + t36147 / F::new(16.0) + t36149 / F::new(48.0) + t36152 + F::new(0.16809375e0) * t31774 + F::new(0.84046875e-1) * t31782 - F::new(0.5603125e-1) * t31790 - t36157 - F::new(0.31448092289604152068e-3) * t36160 + t36163 + F::new(0.42874018118069736972e-3) * t36165 + F::new(0.42874018118069736972e-3) * t36169 + F::new(0.21437009059034868486e-3) * t36173 - t36176 - t36178 - F::new(0.94344276868812456204e-3) * t36181;
    t36183
}
