//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 558/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk558<F: Float>(t816: F, t820: F, t38: F, t531: F, t1477: F, t2060: F, t279: F, t2059: F, t19: F, t2049: F, t796: F, t801: F, t116: F, t784: F, t799: F, t798: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t3259 = t816 * t820;
    let t4258 = t38 * t531;
    let t4259 = 1.0 / t4258;
    let t4339 = t2060 * t1477 * t279;
    let t4340 = t2059 * t4339;
    let t4341 = 0.31636214830824236053e1 * t4340;
    let t4343 = t2049 * t796 * t19;
    let t4344 = t4343 * t801;
    let t4345 = 0.1232289865202e1 * t4344;
    let t4347 = t799 * t784 * t116;
    let t4348 = t798 * t4347;
    (t3259, t4258, t4259, t4339, t4340, t4341, t4343, t4344, t4345, t4347, t4348)
}
