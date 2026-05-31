//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1131/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1131<F: Float>(t19442: F, t34: F, t19453: F, t38: F, t1453: F, t19418: F, t19427: F, t19446: F, t19450: F, t19458: F, t2490: F, t2494: F, t454: F, t4812: F, t4816: F, t4820: F, t4835: F, t6662: F, t6665: F, t974: F, t991: F) -> F {
    let t19545 = F::cast_from(20.0_f64) * t34 * t19442;
    let t19551 = F::cast_from(20.0_f64) * t38 * t19453;
    let t19570 = F::cast_from(50.0_f64) / F::cast_from(81.0_f64) * t991 * t4812 - F::cast_from(25.0_f64) / F::cast_from(9.0_f64) * t991 * t4820 + t19545 + F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t38 * t19446 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t38 * t19450 - t19551 + F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t38 * t19458 + F::cast_from(400.0_f64) / F::cast_from(27.0_f64) * t1453 * t2490 + F::cast_from(200.0_f64) / F::cast_from(9.0_f64) * t1453 * t2494 - F::cast_from(50.0_f64) / F::cast_from(9.0_f64) * t454 * t6662 + F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t34 * t19427 - F::cast_from(50.0_f64) / F::cast_from(9.0_f64) * t991 * t4816 + F::cast_from(10.0_f64) * t38 * t19418 - F::cast_from(2200.0_f64) / F::cast_from(81.0_f64) * t4835 * t974 - F::cast_from(25.0_f64) / F::cast_from(3.0_f64) * t454 * t6665;
    t19570
}
