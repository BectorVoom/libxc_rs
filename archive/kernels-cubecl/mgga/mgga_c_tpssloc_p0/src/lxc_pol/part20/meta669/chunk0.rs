//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2515/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2515<F: Float>(t11243: F, t3271: F, t4756: F, t1102: F, t14758: F, t3270: F, t3287: F, t51000: F, t51004: F, t51007: F, t51010: F, t51012: F, t51014: F, t51016: F, t51018: F, t51021: F) -> (F, F, F, F) {
    let t51024 = t11243 * t4756 * t3271;
    let t51027 = t3270 * t14758 * t1102;
    let t51030 = t3287 * t14758 * t1102;
    let t51032 = F::cast_from(0.543465e1_f64) * t51000 + F::cast_from(0.10064166666666666667e1_f64) * t51004 - F::cast_from(0.485484375e1_f64) * t51007 + F::cast_from(0.6189328125e-1_f64) * t51010 - F::cast_from(0.3883875e1_f64) * t51012 - F::cast_from(0.1294625e1_f64) * t51014 + F::cast_from(0.247573125e0_f64) * t51016 + F::cast_from(0.82524375e-1_f64) * t51018 + F::cast_from(0.58258125e1_f64) * t51021 - F::cast_from(0.1237865625e0_f64) * t51024 - F::cast_from(0.3883875e1_f64) * t51027 + F::cast_from(0.247573125e0_f64) * t51030;
    (t51024, t51027, t51030, t51032)
}
