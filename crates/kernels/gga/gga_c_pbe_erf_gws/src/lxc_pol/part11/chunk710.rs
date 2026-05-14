//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 710/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk710<F: Float>(t1044: F, t10691: F, t1621: F, t1620: F, t2607: F, t3553: F, t1037: F, t11032: F, t2612: F, t3519: F, t1004: F, t3397: F, t184: F, t199: F, t12339: F, t4951: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t12439 = t10691 * t1044;
    let t12440 = t1621 * t12439;
    let t12442 = 4.0 / 5.0 * t1620 * t12440;
    let t12443 = t2607 * t3553;
    let t12444 = t1621 * t12443;
    let t12446 = 4.0 / 5.0 * t1620 * t12444;
    let t12448 = 4.0 / 15.0 * t11032 * t1037;
    let t12450 = 4.0 / 15.0 * t2612 * t3519;
    let t12451 = t3397 * t1004;
    let t12452 = t12451 * t184;
    let t12454 = 4.0 / 5.0 * t12452 * t199;
    let t12460 = t4951 * t12339;
    (t12439, t12440, t12442, t12443, t12444, t12446, t12448, t12450, t12451, t12452, t12454, t12460)
}
