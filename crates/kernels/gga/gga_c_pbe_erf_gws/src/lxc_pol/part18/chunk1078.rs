//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1078/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1078<F: Float>(t51414: F, t2276: F, t2299: F, t3969: F, t876: F, t9246: F, t1185: F, t326: F, t346: F, t6045: F, t2250: F, t51213: F, t14006: F, t6684: F, t816: F, t837: F) -> (F, F, F, F, F, F, F) {
    let t51415 = 595.0 / 5184.0 * t51414;
    let t51421 = t2276 * t3969 * t2299;
    let t51430 = t9246 * t876;
    let t51458 = t326 * t346 * t6045 * t1185;
    let t51459 = 455.0 / 1296.0 * t51458;
    let t51465 = t2250 * t51213;
    let t51470 = t6684 * t14006;
    let t51502 = t816 * t837;
    (t51415, t51421, t51430, t51459, t51465, t51470, t51502)
}
