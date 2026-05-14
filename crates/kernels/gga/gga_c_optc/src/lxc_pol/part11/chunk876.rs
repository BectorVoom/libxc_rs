//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 876/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk876<F: Float>(t11671: F, t14885: F, t14887: F, t14889: F, t17338: F, t17342: F, t17346: F, t17350: F, t17354: F, t17358: F, t8857: F, t415: F, t241: F, t8662: F, t1038: F, t1450: F, t5126: F) -> (F, F, F, F, F, F) {
    let t17360 = -t8857 - 0.12361111111111111111e-1 * t11671 + 0.61805555555555555556e-2 * t14885 - 0.18541666666666666667e-1 * t14887 + 0.92708333333333333334e-2 * t14889 - 0.10300925925925925926e-1 * t17338 + 0.37083333333333333333e-1 * t17342 - 0.18541666666666666666e-1 * t17346 - 0.55625000000000000001e-1 * t17350 + 0.55625000000000000001e-1 * t17354 - 0.92708333333333333333e-2 * t17358;
    let t17361 = t17360 * t415;
    let t17363 = 0.19751789702565206229e-1 * t241 * t17361;
    let t17380 = -t8662 - 4.0 / 9.0 * t11671 + 2.0 / 9.0 * t14885 - 2.0 / 3.0 * t14887 + t14889 / 3.0 - 10.0 / 27.0 * t17338 + 4.0 / 3.0 * t17342 - 2.0 / 3.0 * t17346 - 2.0 * t17350 + 2.0 * t17354 - t17358 / 3.0;
    let t17381 = t1038 * t17380;
    let t17383 = t5126 * t1450;
    (t17360, t17361, t17363, t17380, t17381, t17383)
}
