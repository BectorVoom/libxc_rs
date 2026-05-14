//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 933/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk933<F: Float>(t1864: F, t3668: F, t14687: F, t14689: F, t14691: F, t14693: F, t14696: F, t14698: F, t14701: F, t14704: F, t14708: F, t14710: F, t14712: F, t14715: F, t14719: F, t14722: F, t14724: F, t14727: F, t14729: F, t14731: F, t14733: F, t14736: F) -> (F, F) {
    let t15692 = t1864 * t3668;
    let t15716 = 0.44965277777777777777e-2 * t14687 - 0.14388888888888888889e0 * t14689 - 0.33333333333333333334e0 * t14691 - 0.9375e-1 * t14693 + 0.125e0 * t14696 + 0.33333333333333333334e0 * t14698 - 0.125e0 * t14701 + 0.375e0 * t14704 + 0.89930555555555555554e-2 * t14708 + 0.91666666666666666667e0 * t14710 - 0.5e0 * t14712 - 0.26979166666666666666e-1 * t14715 - 0.53958333333333333333e-1 * t14719 + 0.20234375e-1 * t14722 - 0.34173611111111111111e0 * t14724 + 0.25e0 * t14727 + 0.5e0 * t14729 - 0.20833333333333333333e-1 * t14731 - 0.26979166666666666666e-1 * t14733 + 0.13489583333333333333e-1 * t14736;
    (t15692, t15716)
}
