//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 605/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk605<F: Float>(t3360: F, t87: F, t40: F, t1220: F, t1271: F, t1278: F, t1328: F, t1330: F, t1335: F, t1338: F, t1394: F, t1398: F, t1431: F, t1440: F, t2507: F, t2841: F) -> (F, F, F, F, F) {
    let t3361 = t3360 * t87;
    let t3362 = t40 * t3361;
    let t3364 = -t1394 - t1398 + t1328 - t1431 - t1330 + t1220 - t1271 - t1278 + t1335 + t1338 + t1440;
    let t3365 = 0.36623110073506319882e-3 * t2507;
    let t3366 = 0.11696446794910408142e1 * t2841;
    (t3361, t3362, t3364, t3365, t3366)
}
