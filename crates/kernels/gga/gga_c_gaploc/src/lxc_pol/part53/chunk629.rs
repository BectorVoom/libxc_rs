//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 629/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk629<F: Float>(t13023: F, t1457: F, t2103: F, t3040: F, t3271: F, t3209: F, t8604: F, t1445: F, t5748: F, t2949: F, t813: F, t2958: F, t833: F, t2097: F, t3039: F, t3277: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t13024 = t1457 * t13023;
    let t13026 = 0.71500979903700853338e0 * t2103 * t13024;
    let t13028 = 0.35750489951850426669e0 * t3271 * t3040;
    let t13033 = t8604 * t3209;
    let t13034 = t1445 * t13033;
    let t13036 = 0.27606906686822939767e2 * t5748 * t13034;
    let t13037 = t2949 * t3209;
    let t13038 = t1445 * t13037;
    let t13040 = 0.92023022289409799224e1 * t813 * t13038;
    let t13041 = t2958 * t3209;
    let t13042 = t1445 * t13041;
    let t13044 = 0.43710935587469654631e2 * t833 * t13042;
    let t13045 = t3039 * t2097;
    let t13047 = 0.25025342966295298669e1 * t3277 * t13045;
    (t13024, t13026, t13028, t13033, t13034, t13036, t13037, t13038, t13040, t13041, t13042, t13044, t13045, t13047)
}
