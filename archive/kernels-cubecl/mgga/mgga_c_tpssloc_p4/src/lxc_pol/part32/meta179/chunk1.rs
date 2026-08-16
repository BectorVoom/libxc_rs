//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 886/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk886<F: Float>(t1128: F, t1675: F, t1136: F, t1683: F, t3238: F, t3295: F, t3339: F, t3346: F, t4721: F, t4726: F, t4731: F, t4735: F, t4749: F, t4757: F, t4765: F, t4767: F, t4770: F, t4773: F, t4776: F, t4779: F) -> (F, F, F) {
    let t4797 = t1675 * t1128;
    let t4802 = t1683 * t1136;
    let t4819 = -F::cast_from(0.17648625e1_f64) * t4749 + F::cast_from(0.3529725e1_f64) * t4757 + t3339 - F::cast_from(0.17215833333333333333e0_f64) * t3238 - F::cast_from(0.17215833333333333333e0_f64) * t4721 - F::cast_from(0.34431666666666666667e0_f64) * t4726 + F::cast_from(0.103295e1_f64) * t4731 + F::cast_from(0.516475e0_f64) * t4735 + F::cast_from(0.31558125e0_f64) * t4765 + F::cast_from(0.6311625e0_f64) * t4767 + t3346 - F::cast_from(0.69463333333333333333e-1_f64) * t3295 - F::cast_from(0.69463333333333333333e-1_f64) * t4770 - F::cast_from(0.34731666666666666667e-1_f64) * t4773 + F::cast_from(0.20839e0_f64) * t4776 + F::cast_from(0.104195e0_f64) * t4779;
    (t4797, t4802, t4819)
}
