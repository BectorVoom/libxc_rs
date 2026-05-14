//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 940/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk940<F: Float>(t30468: F, t4425: F, t4685: F, t7822: F, t4331: F, t1470: F, t30644: F, t30984: F, t8458: F, t2268: F, t30456: F, t1562: F, t30948: F, t1444: F, t1992: F, t30154: F, t7586: F) -> (F, F, F, F, F, F, F, F) {
    let t34500 = t30468 * t4425;
    let t34501 = 0.34299214494455789578e-2 * t34500;
    let t34502 = t7822 * t4685;
    let t34504 = t7822 * t4331;
    let t34506 = t30644 * t1470;
    let t34507 = 0.17149607247227894789e-2 * t34506;
    let t34508 = t30984 * t8458;
    let t34510 = t30456 * t2268;
    let t34512 = t30948 * t1562;
    let t34513 = 0.16006300097412701803e-1 * t34512;
    let t34516 = t30154 * t7586 * t1992 * t1444;
    (t34501, t34502, t34504, t34507, t34508, t34510, t34513, t34516)
}
