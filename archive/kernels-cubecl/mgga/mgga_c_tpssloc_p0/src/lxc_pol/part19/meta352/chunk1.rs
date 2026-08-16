//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1279/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1279<F: Float>(t41678: F, t41682: F, t41684: F, t41690: F, t41699: F, t41703: F, t41711: F, t41863: F, t41865: F, t41868: F, t41870: F, t41872: F, t41874: F, t41876: F) -> F {
    let t41878 = -F::cast_from(0.16102666666666666667e1_f64) * t41678 + F::cast_from(0.24154e1_f64) * t41682 + F::cast_from(0.12524296296296296297e1_f64) * t41684 + F::cast_from(0.40256666666666666666e1_f64) * t41690 - F::cast_from(0.72462e1_f64) * t41699 - F::cast_from(0.60384999999999999999e0_f64) * t41703 + F::cast_from(0.72462e1_f64) * t41711 + F::cast_from(0.98115555555555555556e0_f64) * t41863 - F::cast_from(0.44152e0_f64) * t41865 + F::cast_from(0.44152e0_f64) * t41868 - F::cast_from(0.5519e0_f64) * t41870 - F::cast_from(0.18396666666666666667e0_f64) * t41872 + F::cast_from(0.22076e0_f64) * t41874 + F::cast_from(0.98115555555555555555e-1_f64) * t41876;
    t41878
}
