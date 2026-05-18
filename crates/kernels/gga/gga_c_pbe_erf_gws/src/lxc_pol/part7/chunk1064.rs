//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1064/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1064<F: Float>(t159: F, t285: F, t4562: F, t532: F, t545: F, t5676: F, t102: F, t1533: F, t1544: F, t497: F, t5645: F, t413: F, t5772: F, t5773: F) -> (F, F, F, F, F) {
    let t19206 = t532 * t4562 * t159 * t285;
    let t19209 = t5676 * t545 * t285;
    let t19216 = F::new(0.1052289e3) * t102 * t1544 * t1533;
    let t19219 = F::new(0.233842e2) * t102 * t497 * t5645;
    let t19229 = F::new(0.15589466666666666666e2) * t5772 * t5773 * t413;
    (t19206, t19209, t19216, t19219, t19229)
}
