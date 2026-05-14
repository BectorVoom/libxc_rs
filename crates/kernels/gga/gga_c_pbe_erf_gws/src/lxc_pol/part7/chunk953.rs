//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 953/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk953<F: Float>(t102: F, t497: F, t5645: F, t413: F, t5772: F, t5773: F, t5832: F, t5833: F, t481: F, t784: F, t799: F, t5795: F, t119: F, t1533: F, t331: F, t1513: F) -> (F, F, F, F, F, F, F) {
    let t19219 = 0.233842e2 * t102 * t497 * t5645;
    let t19229 = 0.15589466666666666666e2 * t5772 * t5773 * t413;
    let t19232 = 0.26116266666666666667e1 * t5832 * t5833 * t413;
    let t19234 = t799 * t784 * t481;
    let t19235 = t5795 * t19234;
    let t19236 = 0.51964888888888888888e1 * t19235;
    let t19238 = t119 * t331 * t1533;
    let t19239 = t1513 * t19238;
    (t19219, t19229, t19232, t19234, t19236, t19238, t19239)
}
