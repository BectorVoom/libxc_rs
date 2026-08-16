//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1186/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1186(t14738: f64, t14740: f64, t14743: f64, t14745: f64, t14747: f64, t14749: f64, t14751: f64, t14754: f64, t14756: f64, t14760: f64, t14762: f64, t14796: f64, t14856: f64, t14858: f64, t14861: f64, t14863: f64, t14866: f64, t14869: f64, t14872: f64, t14876: f64, t14879: f64, t14882: f64, t15058: f64, t15089: f64) -> f64 {
    let t15092 = t14738 + t14740 / 96.0_f64 + t14743 / 36.0_f64 - t14745 / 16.0_f64 + t14747 / 96.0_f64 + t14749 / 24.0_f64 - t14751 / 8.0_f64 + 11.0_f64 / 27.0_f64 * t14754 + t14756 / 128.0_f64 - t14760 / 256.0_f64 + t14762 / 12.0_f64 + t14796 + t14856 + 2.0_f64 / 27.0_f64 * t14858 - 2.0_f64 / 3.0_f64 * t14861 + t14863 / 24.0_f64 - t14866 / 24.0_f64 + t14869 / 54.0_f64 - t14872 / 288.0_f64 - t14876 / 8.0_f64 + t14879 / 4.0_f64 + t14882 / 3.0_f64 + t15058 / 16.0_f64 + t15089;
    t15092
}
