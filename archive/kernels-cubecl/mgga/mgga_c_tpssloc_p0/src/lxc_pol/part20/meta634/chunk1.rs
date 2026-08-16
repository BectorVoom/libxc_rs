//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2324/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2324<F: Float>(t47681: F, t47686: F, t47691: F, t47695: F, t47699: F, t47703: F, t47706: F, t47707: F, t47709: F, t47711: F, t47713: F, t47715: F, t47717: F, t47722: F, t47724: F, t47728: F, t47731: F, t47732: F, t47736: F, t47738: F) -> F {
    let t47740 = -F::cast_from(0.27469135802469135803e-1_f64) * t47681 + F::cast_from(0.11125e0_f64) * t47686 - F::cast_from(0.18541666666666666666e-1_f64) * t47691 - F::cast_from(0.18541666666666666666e-1_f64) * t47695 - F::cast_from(0.61805555555555555555e-2_f64) * t47699 - F::cast_from(0.166875e0_f64) * t47703 + t47706 - F::cast_from(0.82407407407407407408e-2_f64) * t47707 + F::cast_from(0.12361111111111111111e-1_f64) * t47709 + F::cast_from(0.61805555555555555556e-2_f64) * t47711 + F::cast_from(0.10300925925925925926e-1_f64) * t47713 - F::cast_from(0.37083333333333333333e-1_f64) * t47715 - F::cast_from(0.18541666666666666667e-1_f64) * t47717 - F::cast_from(0.30902777777777777778e-1_f64) * t47722 - F::cast_from(0.37083333333333333334e-1_f64) * t47724 - F::cast_from(0.22249999999999999999e0_f64) * t47728 - t47731 + F::cast_from(0.92708333333333333334e-2_f64) * t47732 - F::cast_from(0.92708333333333333333e-2_f64) * t47736 + F::cast_from(0.55625e-1_f64) * t47738;
    t47740
}
