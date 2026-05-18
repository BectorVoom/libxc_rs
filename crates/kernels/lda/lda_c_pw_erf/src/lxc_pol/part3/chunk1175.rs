//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1175/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1175<F: Float>(t13804: F, t4506: F, t4522: F, t13808: F, t3589: F, t4048: F, t581: F, t13813: F, t11753: F, t11699: F, t11703: F, t11707: F, t11709: F, t11711: F, t11713: F, t11715: F, t11718: F, t11721: F, t11726: F, t11731: F, t11748: F, t11751: F, t11755: F, t11770: F, t11773: F, t11775: F, t11779: F) -> (F, F, F, F) {
    let t13824 = F::new(4.0) / F::new(9.0) * t4506 * t4522 * t13804;
    let t13827 = F::new(4.0) / F::new(9.0) * t4506 * t4522 * t13808;
    let t13829 = t4048 * t581 * t3589;
    let t13832 = F::new(32.0) / F::new(27.0) * t4506 * t13829 * t13813;
    let t13846 = F::new(0.0016792592592592592) * t11753;
    let t13852 = F::new(0.04534) * t11699 - F::new(0.003778333333333333) * t11703 - F::new(0.02267) * t11707 - F::new(0.005037777777777778) * t11709 - F::new(0.0012594444444444445) * t11711 - F::new(0.002099074074074074) * t11713 - F::new(0.02770777777777778) * t11715 + F::new(0.0012594444444444445) * t11718 + F::new(0.007556666666666666) * t11721 + F::new(0.005597530864197531) * t11726 + F::new(0.012594444444444445) * t11731 - F::new(0.04534) * t11748 + F::new(0.003778333333333333) * t11751 + t13846 + F::new(0.007556666666666666) * t11755 + F::new(0.08312333333333333) * t11770 + F::new(0.06801) * t11773 - F::new(0.011335) * t11775 - F::new(0.04534) * t11779;
    (t13824, t13827, t13832, t13852)
}
