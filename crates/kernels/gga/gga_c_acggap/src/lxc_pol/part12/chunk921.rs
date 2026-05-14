//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 921/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk921<F: Float>(t4685: F, t7822: F, t4331: F, t1470: F, t30644: F, t30984: F, t8458: F, t2268: F, t30456: F, t1562: F, t30948: F, t1444: F, t1992: F, t30154: F, t7586: F, t1181: F, t5116: F, t7351: F, t7564: F) -> (F, F, F, F, F, F, F, F) {
    let t34502 = t7822 * t4685;
    let t34504 = t7822 * t4331;
    let t34506 = t30644 * t1470;
    let t34508 = t30984 * t8458;
    let t34510 = t30456 * t2268;
    let t34512 = t30948 * t1562;
    let t34516 = t30154 * t7586 * t1992 * t1444;
    let t34522 = t7564 * t1181 * t7351 * t5116;
    (t34502, t34504, t34506, t34508, t34510, t34512, t34516, t34522)
}
