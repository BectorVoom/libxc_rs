//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 898/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk898<F: Float>(t3074: F, t8880: F, t2138: F, t1133: F, t2157: F, t810: F, t874: F, t4386: F, t3138: F, t1105: F, t2171: F, t8599: F, t2168: F, t1134: F, t858: F, t2407: F) -> (F, F, F, F, F, F, F, F) {
    let t8881 = t3074 * t8880;
    let t8883 = t8881 * t2138 / 48.0;
    let t8884 = t1133 * t2157;
    let t8885 = t874 * t810;
    let t8886 = t8884 * t8885;
    let t8887 = t4386 * t8886;
    let t8889 = t3138 * t8887 / 12.0;
    let t8890 = t1105 * t874;
    let t8891 = t8890 * t2171;
    let t8892 = t8599 * t8891;
    let t8894 = t2168 * t8892 / 8.0;
    let t8895 = t1134 * t810;
    let t8896 = t858 * t8895;
    let t8897 = t2407 * t8896;
    (t8883, t8884, t8886, t8889, t8890, t8891, t8894, t8897)
}
