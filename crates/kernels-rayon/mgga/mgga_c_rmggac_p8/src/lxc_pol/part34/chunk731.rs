//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 731/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk731(t14103: f64, t14642: f64, t14138: f64, t14142: f64, t14205: f64, t14686: f64, t14697: f64, t14272: f64, t14396: f64, t16156: f64, t3219: f64, t34881: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t70714 = 0.3313366304663878551e-1_f64 * t14103;
    let t70720 = 0.19863479950205658386e-4_f64 * t14642;
    let t70721 = 0.60975299583150056624e-3_f64 * t14138;
    let t70722 = 0.86737941314158990616e-4_f64 * t14142;
    let t70735 = 0.162600798888400151e-2_f64 * t14205;
    let t70741 = 0.79828278012425390426e-1_f64 * t14686;
    let t70745 = 0.30487649791575028314e-3_f64 * t14697;
    let t70746 = 0.17347588262831798124e-3_f64 * t14272;
    let t70748 = t16156 * t14396;
    let t70754 = t34881 * t3219;
    (t70714, t70720, t70721, t70722, t70735, t70741, t70745, t70746, t70748, t70754)
}
