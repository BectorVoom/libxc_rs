//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1137/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1137<F: Float>(t1125: F, t51317: F, t4039: F, t9411: F, t28139: F, t850: F, t14093: F, t51306: F, t9609: F, t3065: F, t3167: F, t2134: F, t3253: F, t51255: F, t14099: F, t863: F, t885: F) -> (F, F, F, F, F, F, F) {
    let t54075 = t1125 * t51317;
    let t54077 = t4039 * t9411;
    let t54079 = t850 * t28139;
    let t54080 = t54079 * t14093;
    let t54082 = t51306 * t9609;
    let t54084 = t3065 * t3167;
    let t54085 = t2134 * t54084;
    let t54087 = t51255 * t3253;
    let t54088 = 7.0 / 144.0 * t54087;
    let t54090 = t863 * t14099 * t885;
    (t54075, t54077, t54080, t54082, t54085, t54088, t54090)
}
