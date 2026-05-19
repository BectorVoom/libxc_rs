//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 333/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk333<F: Float>(t1275: F, t1293: F, t1291: F, t155: F, t449: F, t1215: F, t75: F, t1216: F, t456: F) -> (F, F, F, F, F) {
    let t1294 = t1275 * t1293;
    let t1295 = t1291 * t1294;
    let t1296 = F::cast_from(0.16081824322151104822e2_f64) * t1295;
    let t1300 = t155 * t449;
    let t1304 = t75 * t1215;
    let t1305 = t1216 * t456;
    (t1294, t1296, t1300, t1304, t1305)
}
