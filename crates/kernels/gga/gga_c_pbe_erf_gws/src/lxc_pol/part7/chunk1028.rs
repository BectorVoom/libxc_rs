//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1028/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1028<F: Float>(t1392: F, t1447: F, t542: F, t156: F, t4749: F, t409: F, t4745: F, t414: F, t1285: F, t1290: F, t1293: F, t395: F, t403: F) -> (F, F, F, F, F) {
    let t18607 = F::new(0.1284251895870376528e1) * t1447 * t542 * t1392;
    let t18610 = F::new(0.38527556876111295841e1) * t1447 * t156 * t4749;
    let t18611 = t409 * t4745;
    let t18612 = F::new(48.0) * t18611;
    let t18613 = t414 * t4745;
    let t18614 = F::new(48.0) * t18613;
    let t18619 = F::new(0.34366858576436911004e1) * t395 * t1290 * t1285 * t1293 * t403;
    (t18607, t18610, t18612, t18614, t18619)
}
