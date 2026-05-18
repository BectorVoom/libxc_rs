//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1162/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1162<F: Float>(t14687: F, t14689: F, t14691: F, t14693: F, t14696: F, t14698: F, t14701: F, t14704: F, t14708: F, t14710: F, t14712: F, t14715: F, t14719: F, t14722: F, t14724: F, t14727: F, t14729: F, t14731: F, t14733: F, t14736: F) -> F {
    let t14738 = t14687 / F::new(576.0) - t14689 / F::new(18.0) - F::new(2.0) / F::new(9.0) * t14691 - t14693 / F::new(16.0) + t14696 / F::new(12.0) + F::new(2.0) / F::new(9.0) * t14698 - t14701 / F::new(12.0) + t14704 / F::new(4.0) + t14708 / F::new(288.0) + F::new(11.0) / F::new(18.0) * t14710 - t14712 / F::new(3.0) - t14715 / F::new(96.0) - t14719 / F::new(48.0) + t14722 / F::new(128.0) - F::new(19.0) / F::new(144.0) * t14724 + t14727 / F::new(6.0) + t14729 / F::new(3.0) - t14731 / F::new(72.0) - t14733 / F::new(96.0) + t14736 / F::new(192.0);
    t14738
}
