//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 764/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk764(t11709: f64, t11712: f64, t11715: f64, t11718: f64, t11721: f64, t11724: f64, t11728: f64, t11732: f64, t11736: f64, t11739: f64, t11742: f64, t11745: f64, t11747: f64, t11751: f64, t11754: f64, t11756: f64, t11758: f64, t11760: f64) -> f64 {
    let t11762 = -t11709 / 8.0_f64 + t11712 / 4.0_f64 - t11715 / 8.0_f64 + t11718 / 32.0_f64 + 3.0_f64 / 8.0_f64 * t11721 - t11724 / 32.0_f64 - t11728 / 256.0_f64 + 3.0_f64 / 8.0_f64 * t11732 + t11736 / 256.0_f64 + t11739 / 24.0_f64 - 3.0_f64 / 128.0_f64 * t11742 + 3.0_f64 / 256.0_f64 * t11745 - t11747 / 192.0_f64 + t11751 / 24.0_f64 + t11754 / 64.0_f64 - t11756 / 64.0_f64 + t11758 / 8.0_f64 - t11760 / 64.0_f64;
    t11762
}
