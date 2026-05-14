//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 900/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk900<F: Float>(t35410: F, t435: F, t7815: F, t2299: F, t7780: F, t7637: F, t8545: F, t1429: F, t7614: F, t1413: F, t7685: F, t1441: F, t30984: F, t8649: F, t30934: F, t8602: F) -> (F, F, F, F, F, F, F, F, F) {
    let t35411 = t35410 / 96.0;
    let t35413 = t7815 * t435;
    let t35418 = t7780 * t2299;
    let t35425 = t7637 * t8545;
    let t35436 = t7614 * t1429;
    let t35447 = t7685 * t1413;
    let t35448 = 0.40015750243531754508e-2 * t35447;
    let t35451 = t7614 * t1441;
    let t35452 = 0.32012600194825403606e-1 * t35451;
    let t35456 = t30984 * t8649;
    let t35458 = t30934 * t8602;
    (t35411, t35413, t35418, t35425, t35436, t35448, t35452, t35456, t35458)
}
