//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 634/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk634<F: Float>(t1260: F, t6837: F, t286: F, t1251: F, t3499: F, t3514: F, t5300: F, t5322: F, t6759: F, t6763: F, t6767: F, t6771: F, t6776: F) -> (F, F, F) {
    let t6838 = t1260 * t6837;
    let t6839 = t286 * t6838;
    let t6842 = -t3499 + t5300 / F::cast_from(864.0_f64) - t5322 / F::cast_from(288.0_f64) + t1251 * t6759 / F::cast_from(432.0_f64) - t3514 * t6763 / F::cast_from(288.0_f64) - t1251 * t6767 / F::cast_from(288.0_f64) + t1251 * t6771 / F::cast_from(576.0_f64) + t1251 * t6776 / F::cast_from(96.0_f64) - t1251 * t6839 / F::cast_from(192.0_f64);
    (t6838, t6839, t6842)
}
