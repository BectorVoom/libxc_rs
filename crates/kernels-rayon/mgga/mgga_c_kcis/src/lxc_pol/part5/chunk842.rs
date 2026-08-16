//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 842/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk842(t6751: f64, t6855: f64, t1872: f64, t6683: f64, t6685: f64, t6687: f64, t6691: f64, t6694: f64, t6698: f64, t6702: f64, t6706: f64, t6710: f64, t6712: f64, t6714: f64, t6718: f64, t6721: f64, t6725: f64, t6729: f64, t6733: f64) -> (f64, f64, f64) {
    let t6856 = t6751 + t6855;
    let t6860 = t1872 * t1872;
    let t6879 = 0.9375e-1_f64 * t6683 - 0.1875e0_f64 * t6685 + 0.125e0_f64 * t6687 + 0.1875e0_f64 * t6691 - 0.125e0_f64 * t6694 - 0.9375e-1_f64 * t6698 - 0.20833333333333333333e-1_f64 * t6702 + 0.625e-1_f64 * t6706 - 0.101171875e-1_f64 * t6710 + 0.20234375e-1_f64 * t6712 - 0.26979166666666666666e-1_f64 * t6714 - 0.20234375e-1_f64 * t6718 + 0.26979166666666666666e-1_f64 * t6721 + 0.101171875e-1_f64 * t6725 - 0.44965277777777777777e-2_f64 * t6729 - 0.13489583333333333333e-1_f64 * t6733;
    (t6856, t6860, t6879)
}
