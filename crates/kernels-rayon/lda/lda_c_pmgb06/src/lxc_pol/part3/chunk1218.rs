//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1218/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1218(t10743: f64, t10746: f64, t10748: f64, t10751: f64, t10754: f64, t10757: f64, t10760: f64, t10764: f64, t10769: f64, t10770: f64, t10773: f64, t10777: f64, t13829: f64, t13834: f64, t13835: f64, t13837: f64, t13839: f64, t13841: f64, t13843: f64, t13845: f64, t13847: f64, t13849: f64, t13851: f64) -> (f64, f64) {
    let t14453 = 0.6492624817418906_f64 * t10743 + t10746 + 0.10821041362364843_f64 * t10748 + 0.6492624817418906_f64 * t10751 + 0.03354522822333102_f64 * t10754 + 0.9738937226128359_f64 * t10757 + 0.10063568466999305_f64 * t10760 + t10764 + t10769 - 0.2885611029963958_f64 * t10770 - t10773;
    let t14454 = t10777 + t13829 + t13834 - t13835 - t13837 - t13839 - t13841 + t13843 + t13845 + t13847 + t13849 + t13851;
    (t14453, t14454)
}
