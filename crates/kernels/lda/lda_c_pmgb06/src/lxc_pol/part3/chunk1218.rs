//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1218/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1218<F: Float>(t10743: F, t10746: F, t10748: F, t10751: F, t10754: F, t10757: F, t10760: F, t10764: F, t10769: F, t10770: F, t10773: F, t10777: F, t13829: F, t13834: F, t13835: F, t13837: F, t13839: F, t13841: F, t13843: F, t13845: F, t13847: F, t13849: F, t13851: F) -> (F, F) {
    let t14453 = F::new(0.6492624817418906) * t10743 + t10746 + F::new(0.10821041362364843) * t10748 + F::new(0.6492624817418906) * t10751 + F::new(0.03354522822333102) * t10754 + F::new(0.9738937226128359) * t10757 + F::new(0.10063568466999305) * t10760 + t10764 + t10769 - F::new(0.2885611029963958) * t10770 - t10773;
    let t14454 = t10777 + t13829 + t13834 - t13835 - t13837 - t13839 - t13841 + t13843 + t13845 + t13847 + t13849 + t13851;
    (t14453, t14454)
}
