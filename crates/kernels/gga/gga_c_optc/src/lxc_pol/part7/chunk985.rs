//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 985/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk985<F: Float>(t104: F, t1928: F, t22134: F, t22136: F, t22141: F, t22143: F, t22152: F, t22274: F, t22277: F, t22281: F, t23281: F, t3539: F, t6312: F, t6704: F, t714: F, t95: F) -> (F,) {
    let t23286 = -t22134 - t22136 - 0.93041573165652349788e-1 * t3539 * t6704 * t1928 + t22141 - t22143 + 0.93041573165652349788e-1 * t3539 * t6312 * t1928 + t22152 + 0.25844881434903430496e-2 * t95 * t104 * t23281 * t714 + t22274 + t22277 + t22281;
    (t23286,)
}
