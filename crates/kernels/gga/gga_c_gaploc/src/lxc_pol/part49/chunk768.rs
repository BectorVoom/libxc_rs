//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 768/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk768<F: Float>(t13034: F, t5748: F, t2949: F, t3209: F, t1445: F, t813: F, t2958: F, t833: F, t2097: F, t3039: F, t3277: F, t12658: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t13036 = F::new(0.27606906686822939767e2) * t5748 * t13034;
    let t13037 = t2949 * t3209;
    let t13038 = t1445 * t13037;
    let t13040 = F::new(0.92023022289409799224e1) * t813 * t13038;
    let t13041 = t2958 * t3209;
    let t13042 = t1445 * t13041;
    let t13044 = F::new(0.43710935587469654631e2) * t833 * t13042;
    let t13045 = t3039 * t2097;
    let t13047 = F::new(0.25025342966295298669e1) * t3277 * t13045;
    let t13050 = F::new(0.11502877786176224903e1) * t12658;
    (t13036, t13037, t13038, t13040, t13041, t13042, t13044, t13045, t13047, t13050)
}
