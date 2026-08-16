//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1033/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1033<F: Float>(t1864: F, t3668: F, t14687: F, t14689: F, t14691: F, t14693: F, t14696: F, t14698: F, t14701: F, t14704: F, t14708: F, t14710: F, t14712: F, t14715: F, t14719: F, t14722: F, t14724: F, t14727: F, t14729: F, t14731: F, t14733: F, t14736: F) -> (F, F) {
    let t15692 = t1864 * t3668;
    let t15716 = F::cast_from(0.44965277777777777777e-2_f64) * t14687 - F::cast_from(0.14388888888888888889e0_f64) * t14689 - F::cast_from(0.33333333333333333334e0_f64) * t14691 - F::cast_from(0.9375e-1_f64) * t14693 + F::cast_from(0.125e0_f64) * t14696 + F::cast_from(0.33333333333333333334e0_f64) * t14698 - F::cast_from(0.125e0_f64) * t14701 + F::cast_from(0.375e0_f64) * t14704 + F::cast_from(0.89930555555555555554e-2_f64) * t14708 + F::cast_from(0.91666666666666666667e0_f64) * t14710 - F::cast_from(0.5e0_f64) * t14712 - F::cast_from(0.26979166666666666666e-1_f64) * t14715 - F::cast_from(0.53958333333333333333e-1_f64) * t14719 + F::cast_from(0.20234375e-1_f64) * t14722 - F::cast_from(0.34173611111111111111e0_f64) * t14724 + F::cast_from(0.25e0_f64) * t14727 + F::cast_from(0.5e0_f64) * t14729 - F::cast_from(0.20833333333333333333e-1_f64) * t14731 - F::cast_from(0.26979166666666666666e-1_f64) * t14733 + F::cast_from(0.13489583333333333333e-1_f64) * t14736;
    (t15692, t15716)
}
