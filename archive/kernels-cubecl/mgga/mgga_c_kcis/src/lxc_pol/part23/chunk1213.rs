//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1213/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1213<F: Float>(t17460: F, t7948: F, t97742: F, t97744: F, t97746: F, t97748: F, t97750: F, t97752: F, t97754: F, t97756: F, t97758: F, t97760: F, t97762: F, t97765: F, t97768: F, t97770: F, t97773: F, t97775: F, t97777: F) -> (F, F) {
    let t97779 = t7948 * t17460;
    let t97781 = F::cast_from(0.17986111111111111111e-1_f64) * t97742 + F::cast_from(0.33333333333333333334e0_f64) * t97744 - F::cast_from(0.21583333333333333334e0_f64) * t97746 - F::cast_from(0.625e-1_f64) * t97748 + F::cast_from(0.20234375e-1_f64) * t97750 + F::cast_from(0.59953703703703703705e-2_f64) * t97752 - F::cast_from(0.4046875e-1_f64) * t97754 - F::cast_from(0.20833333333333333333e-1_f64) * t97756 - F::cast_from(0.10791666666666666667e0_f64) * t97758 + F::cast_from(0.53958333333333333334e-1_f64) * t97760 + F::cast_from(0.53958333333333333334e-1_f64) * t97762 - F::cast_from(0.809375e-1_f64) * t97765 + F::cast_from(0.53958333333333333334e-1_f64) * t97768 - F::cast_from(0.89930555555555555557e-2_f64) * t97770 + F::cast_from(0.125e0_f64) * t97773 - F::cast_from(0.125e0_f64) * t97775 - F::cast_from(0.53958333333333333334e-1_f64) * t97777 - F::cast_from(0.625e-1_f64) * t97779;
    (t97779, t97781)
}
