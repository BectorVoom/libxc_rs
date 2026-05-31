//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 954/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk954<F: Float>(t3340: F, t995: F, t3356: F, t3323: F, t10415: F, t10418: F, t10423: F, t10438: F, t10441: F, t10445: F, t28: F, t3330: F, t3334: F, t3347: F, t34: F, t38: F, t984: F, t991: F, tau1: F) -> (F, F, F, F) {
    let t10451 = t3340 * t995;
    let t10454 = t995 * t3356;
    let t10463 = tau1 * t3323;
    let t10478 = -F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t34 * t10415 + F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t34 * t10418 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t34 * t10423 - F::cast_from(440.0_f64) / F::cast_from(27.0_f64) * t10463 * t28 + F::cast_from(200.0_f64) / F::cast_from(9.0_f64) * t3347 * t984 - F::cast_from(50.0_f64) / F::cast_from(9.0_f64) * t991 * t3330 - F::cast_from(25.0_f64) / F::cast_from(3.0_f64) * t991 * t3334 - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t38 * t10438 + F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t38 * t10441 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t38 * t10445;
    (t10451, t10454, t10463, t10478)
}
