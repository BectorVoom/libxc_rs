//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 925/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk925<F: Float>(t18611: F, t414: F, t4745: F, t1285: F, t1290: F, t1293: F, t395: F, t403: F, t18587: F, t18589: F, t18591: F, t18594: F, t18596: F, t18599: F, t18601: F, t18604: F, t18607: F, t18610: F) -> (F, F, F, F) {
    let t18612 = 48.0 * t18611;
    let t18613 = t414 * t4745;
    let t18614 = 48.0 * t18613;
    let t18619 = 0.34366858576436911004e1 * t395 * t1290 * t1285 * t1293 * t403;
    let t18620 = t18587 - t18589 + t18591 + t18594 + t18596 + t18599 - t18601 - t18604 - t18607 - t18610 + t18612 - t18614 - t18619;
    (t18612, t18614, t18619, t18620)
}
