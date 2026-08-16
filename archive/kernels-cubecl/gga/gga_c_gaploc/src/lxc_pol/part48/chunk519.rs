//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 519/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk519<F: Float>(t9848: F, t2465: F, t2571: F, t2464: F, t825: F, t1645: F, t9740: F, t2194: F, t3308: F, t7068: F, t883: F, t1967: F) -> (F, F, F, F, F, F) {
    let t9849 = F::cast_from(0.38342925953920749676e0_f64) * t9848;
    let t9850 = t2465 * t2571;
    let t9851 = t2464 * t9850;
    let t9852 = t825 * t9851;
    let t9853 = F::cast_from(0.85206502119823888169e-1_f64) * t9852;
    let t9858 = t1645 * t9740;
    let t9873 = t2194 * t3308;
    let t9889 = t883 * t7068;
    let t9890 = t1967 * t9889;
    (t9849, t9853, t9858, t9873, t9889, t9890)
}
