//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1175/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1175(t13804: f64, t4506: f64, t4522: f64, t13808: f64, t3589: f64, t4048: f64, t581: f64, t13813: f64, t11753: f64, t11699: f64, t11703: f64, t11707: f64, t11709: f64, t11711: f64, t11713: f64, t11715: f64, t11718: f64, t11721: f64, t11726: f64, t11731: f64, t11748: f64, t11751: f64, t11755: f64, t11770: f64, t11773: f64, t11775: f64, t11779: f64) -> (f64, f64, f64, f64) {
    let t13824 = 4.0_f64 / 9.0_f64 * t4506 * t4522 * t13804;
    let t13827 = 4.0_f64 / 9.0_f64 * t4506 * t4522 * t13808;
    let t13829 = t4048 * t581 * t3589;
    let t13832 = 32.0_f64 / 27.0_f64 * t4506 * t13829 * t13813;
    let t13846 = 0.0016792592592592592_f64 * t11753;
    let t13852 = 0.04534_f64 * t11699 - 0.003778333333333333_f64 * t11703 - 0.02267_f64 * t11707 - 0.005037777777777778_f64 * t11709 - 0.0012594444444444445_f64 * t11711 - 0.002099074074074074_f64 * t11713 - 0.02770777777777778_f64 * t11715 + 0.0012594444444444445_f64 * t11718 + 0.007556666666666666_f64 * t11721 + 0.005597530864197531_f64 * t11726 + 0.012594444444444445_f64 * t11731 - 0.04534_f64 * t11748 + 0.003778333333333333_f64 * t11751 + t13846 + 0.007556666666666666_f64 * t11755 + 0.08312333333333333_f64 * t11770 + 0.06801_f64 * t11773 - 0.011335_f64 * t11775 - 0.04534_f64 * t11779;
    (t13824, t13827, t13832, t13852)
}
