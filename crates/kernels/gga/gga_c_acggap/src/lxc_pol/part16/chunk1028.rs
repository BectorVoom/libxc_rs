//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1028/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1028<F: Float>(t36156: F, t30120: F, t8948: F, t7839: F, t8787: F, t30689: F, t5286: F, t1181: F, t22275: F, t604: F, t7493: F, t1165: F, t23745: F, t7351: F) -> (F, F, F, F, F, F) {
    let t36157 = F::new(0.62896184579208304136e-3) * t36156;
    let t36162 = t30120 * t8948;
    let t36163 = F::new(0.42874018118069736972e-3) * t36162;
    let t36175 = t7839 * t8787;
    let t36176 = F::new(0.94344276868812456204e-3) * t36175;
    let t36177 = t30689 * t5286;
    let t36178 = F::new(0.34299214494455789578e-2) * t36177;
    let t36194 = t7493 * t1181 * t604 * t22275;
    let t36195 = F::new(0.31448092289604152068e-2) * t36194;
    let t36198 = t7493 * t1165 * t7351 * t23745;
    (t36157, t36163, t36176, t36178, t36195, t36198)
}
