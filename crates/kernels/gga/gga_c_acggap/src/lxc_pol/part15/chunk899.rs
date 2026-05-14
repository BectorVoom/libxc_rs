//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 899/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk899<F: Float>(t1298: F, t7380: F, t7381: F, t1524: F, t1983: F, t2095: F, t435: F, t7815: F, t2299: F, t7780: F, t31231: F, t7637: F, t8545: F, t1429: F, t7614: F, t1413: F, t7685: F) -> (F, F, F, F, F, F, F, F) {
    let t35407 = t7380 * t7381 * t1298;
    let t35410 = t2095 * t1983 * t1524;
    let t35413 = t7815 * t435;
    let t35418 = t7780 * t2299;
    let t35424 = 0.34299214494455789578e-2 * t31231;
    let t35425 = t7637 * t8545;
    let t35436 = t7614 * t1429;
    let t35447 = t7685 * t1413;
    (t35407, t35410, t35413, t35418, t35424, t35425, t35436, t35447)
}
