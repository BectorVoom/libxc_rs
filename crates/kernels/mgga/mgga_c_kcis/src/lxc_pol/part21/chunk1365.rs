//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1365/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1365<F: Float>(t26966: F, t28214: F, t15198: F, t2197: F, t27014: F, t27020: F, t28146: F, t28190: F, t7779: F, t8091: F, t92613: F, t92991: F, t92993: F, t92997: F, t96042: F, t96045: F, t96048: F, t96052: F) -> F {
    let t97212 = t26966 * t28214;
    let t97215 = F::new(0.77382407407407407407e-3) * t92991 + F::new(0.77382407407407407406e-3) * t92993 - F::new(0.51588271604938271604e-3) * t92997 + F::new(0.92858888888888888886e-2) * t96042 + F::new(0.34752604166666666667e-3) * t28190 * t27020 + F::new(0.18534722222222222222e-2) * t15198 * t7779 * t2197 - F::new(0.92858888888888888888e-2) * t96045 - F::new(0.46336805555555555556e-3) * t27014 * t28146 - F::new(0.11326774691358024691e-2) * t92613 * t8091 - F::new(0.15476481481481481481e-2) * t96048 + F::new(0.20594135802469135802e-3) * t97212 + F::new(0.23214722222222222222e-2) * t96052;
    t97215
}
