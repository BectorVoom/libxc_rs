//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1131/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1131(t19442: f64, t34: f64, t19453: f64, t38: f64, t1453: f64, t19418: f64, t19427: f64, t19446: f64, t19450: f64, t19458: f64, t2490: f64, t2494: f64, t454: f64, t4812: f64, t4816: f64, t4820: f64, t4835: f64, t6662: f64, t6665: f64, t974: f64, t991: f64) -> f64 {
    let t19545 = 20.0_f64 * t34 * t19442;
    let t19551 = 20.0_f64 * t38 * t19453;
    let t19570 = 50.0_f64 / 81.0_f64 * t991 * t4812 - 25.0_f64 / 9.0_f64 * t991 * t4820 + t19545 + 40.0_f64 / 81.0_f64 * t38 * t19446 - 10.0_f64 / 3.0_f64 * t38 * t19450 - t19551 + 10.0_f64 / 9.0_f64 * t38 * t19458 + 400.0_f64 / 27.0_f64 * t1453 * t2490 + 200.0_f64 / 9.0_f64 * t1453 * t2494 - 50.0_f64 / 9.0_f64 * t454 * t6662 + 10.0_f64 / 9.0_f64 * t34 * t19427 - 50.0_f64 / 9.0_f64 * t991 * t4816 + 10.0_f64 * t38 * t19418 - 2200.0_f64 / 81.0_f64 * t4835 * t974 - 25.0_f64 / 3.0_f64 * t454 * t6665;
    t19570
}
