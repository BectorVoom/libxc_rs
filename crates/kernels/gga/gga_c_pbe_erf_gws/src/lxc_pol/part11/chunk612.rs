//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 612/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk612<F: Float>(t164: F, t5984: F, t1964: F, t528: F, t547: F, t762: F, t1464: F, t163: F, t169: F, t234: F, t366: F, t1479: F, t553: F, t1371: F, t1952: F, t1378: F, t1971: F, t5697: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5985 = t5984 * t164;
    let t5986 = 0.1186530987165140469e-3 * t5985;
    let t5988 = 0.94516221669423353502e-1 * t528 * t1964;
    let t5993 = 0.18903244333884670701e0 * t762 * t547;
    let t5999 = 0.189032443338846707e0 * t1464 * t164;
    let t6003 = 0.87811049408533800023e-1 * t169 * t366 * t234 * t163;
    let t6005 = 0.258995450979035416e-1 * t1479 * t553;
    let t6012 = 0.19753890328909480882e-1 * t1952 * t1371 * t553;
    let t6015 = 0.34679929861433484636e-2 * t5697 * t1378 * t1971;
    (t5985, t5986, t5988, t5993, t5999, t6003, t6005, t6012, t6015)
}
