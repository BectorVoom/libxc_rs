//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2114/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2114<F: Float>(t584: F, t86730: F, t868: F, t25372: F, t193: F, t201: F, t7540: F, t200: F, t6665: F, t4303: F, t606: F, t1877: F, t1915: F, t9212: F) -> (F, F, F, F, F) {
    let t86732 = t86730 * t584 * t868;
    let t86734 = F::cast_from(2.0_f64) * t25372 * t86732;
    let t86736 = t193 * t201 * t7540;
    let t86740 = t193 * t200 * t6665;
    let t86746 = t606 * t4303;
    let t86751 = F::cast_from(3.0_f64) * t1877 * t1915 * t9212;
    (t86734, t86736, t86740, t86746, t86751)
}
