//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 798/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk798<F: Float>(t3106: F, t360: F, t2306: F, t3074: F, t1133: F, t2157: F, t1105: F, t874: F, t1134: F, t810: F, t858: F, t2407: F, t2142: F, t3120: F, t332: F, t6238: F, t863: F) -> (F, F, F, F, F, F, F) {
    let t8879 = t3106 * t360;
    let t8880 = t2306 * t8879;
    let t8881 = t3074 * t8880;
    let t8884 = t1133 * t2157;
    let t8890 = t1105 * t874;
    let t8895 = t1134 * t810;
    let t8896 = t858 * t8895;
    let t8897 = t2407 * t8896;
    let t8901 = 7.0 / 144.0 * t3120 * t2142;
    let t8903 = t863 * t6238 * t332;
    (t8881, t8884, t8890, t8895, t8897, t8901, t8903)
}
