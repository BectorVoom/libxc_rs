//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 708/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk708<F: Float>(t13034: F, t5748: F, t2949: F, t3209: F, t1445: F, t813: F, t2958: F, t833: F, t2097: F, t3039: F, t3277: F, t13010: F, t13015: F, t13018: F, t13021: F, t13026: F, t13028: F, t13029: F, t13031: F) -> (F, F, F, F, F, F) {
    let t13036 = F::cast_from(0.27606906686822939767e2_f64) * t5748 * t13034;
    let t13037 = t2949 * t3209;
    let t13038 = t1445 * t13037;
    let t13040 = F::cast_from(0.92023022289409799224e1_f64) * t813 * t13038;
    let t13041 = t2958 * t3209;
    let t13042 = t1445 * t13041;
    let t13044 = F::cast_from(0.43710935587469654631e2_f64) * t833 * t13042;
    let t13045 = t3039 * t2097;
    let t13047 = F::cast_from(0.25025342966295298669e1_f64) * t3277 * t13045;
    let t13048 = -F::cast_from(0.13803453343411469884e2_f64) * t13010 - t13015 - t13018 + F::cast_from(0.14300195980740170668e1_f64) * t13021 + t13026 + t13028 + F::cast_from(0.71500979903700853338e0_f64) * t13029 - F::cast_from(0.21450293971110256002e1_f64) * t13031 + t13036 - t13040 + t13044 - t13047;
    (t13037, t13038, t13041, t13042, t13045, t13048)
}
