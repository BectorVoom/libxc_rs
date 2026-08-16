//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1099/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1099<F: Float>(t5727: F, t912: F, t2792: F, t2844: F, t5726: F, t2842: F, t4395: F, t4399: F, t10704: F, t5694: F, t10702: F, t5743: F, t931: F) -> (F, F, F, F, F) {
    let t17517 = t5727 * t912;
    let t17519 = F::cast_from(2.0_f64) * t2792 * t17517;
    let t17520 = t5726 * t2844;
    let t17521 = t17520 * t912;
    let t17523 = F::cast_from(0.16081979498692535067e2_f64) * t2842 * t17521;
    let t17524 = t4399 * t4395;
    let t17526 = F::cast_from(0.32163958997385070134e2_f64) * t2842 * t17524;
    let t17527 = t5694 * t10704;
    let t17528 = t17527 * t912;
    let t17530 = F::cast_from(0.51726012919273400301e3_f64) * t10702 * t17528;
    let t17535 = t5743 * t931;
    (t17519, t17523, t17526, t17530, t17535)
}
