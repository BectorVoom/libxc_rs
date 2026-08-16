//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1186/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1186<F: Float>(t14738: F, t14740: F, t14743: F, t14745: F, t14747: F, t14749: F, t14751: F, t14754: F, t14756: F, t14760: F, t14762: F, t14796: F, t14856: F, t14858: F, t14861: F, t14863: F, t14866: F, t14869: F, t14872: F, t14876: F, t14879: F, t14882: F, t15058: F, t15089: F) -> F {
    let t15092 = t14738 + t14740 / F::cast_from(96.0_f64) + t14743 / F::cast_from(36.0_f64) - t14745 / F::cast_from(16.0_f64) + t14747 / F::cast_from(96.0_f64) + t14749 / F::cast_from(24.0_f64) - t14751 / F::cast_from(8.0_f64) + F::cast_from(11.0_f64) / F::cast_from(27.0_f64) * t14754 + t14756 / F::cast_from(128.0_f64) - t14760 / F::cast_from(256.0_f64) + t14762 / F::cast_from(12.0_f64) + t14796 + t14856 + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t14858 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t14861 + t14863 / F::cast_from(24.0_f64) - t14866 / F::cast_from(24.0_f64) + t14869 / F::cast_from(54.0_f64) - t14872 / F::cast_from(288.0_f64) - t14876 / F::cast_from(8.0_f64) + t14879 / F::cast_from(4.0_f64) + t14882 / F::cast_from(3.0_f64) + t15058 / F::cast_from(16.0_f64) + t15089;
    t15092
}
