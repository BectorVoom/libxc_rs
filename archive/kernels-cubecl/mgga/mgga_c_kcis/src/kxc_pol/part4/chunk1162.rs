//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1162/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1162<F: Float>(t14687: F, t14689: F, t14691: F, t14693: F, t14696: F, t14698: F, t14701: F, t14704: F, t14708: F, t14710: F, t14712: F, t14715: F, t14719: F, t14722: F, t14724: F, t14727: F, t14729: F, t14731: F, t14733: F, t14736: F) -> F {
    let t14738 = t14687 / F::cast_from(576.0_f64) - t14689 / F::cast_from(18.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t14691 - t14693 / F::cast_from(16.0_f64) + t14696 / F::cast_from(12.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t14698 - t14701 / F::cast_from(12.0_f64) + t14704 / F::cast_from(4.0_f64) + t14708 / F::cast_from(288.0_f64) + F::cast_from(11.0_f64) / F::cast_from(18.0_f64) * t14710 - t14712 / F::cast_from(3.0_f64) - t14715 / F::cast_from(96.0_f64) - t14719 / F::cast_from(48.0_f64) + t14722 / F::cast_from(128.0_f64) - F::cast_from(19.0_f64) / F::cast_from(144.0_f64) * t14724 + t14727 / F::cast_from(6.0_f64) + t14729 / F::cast_from(3.0_f64) - t14731 / F::cast_from(72.0_f64) - t14733 / F::cast_from(96.0_f64) + t14736 / F::cast_from(192.0_f64);
    t14738
}
