//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1217/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1217(t13771: f64, t13774: f64, t13776: f64, t13781: f64, t13783: f64, t13787: f64, t13793: f64, t13795: f64, t13799: f64, t13801: f64, t13803: f64, t13806: f64, t13808: f64, t13810: f64, t13812: f64, t13816: f64, t13817: f64, t13818: f64, t13819: f64, t13820: f64, t13822: f64, t13823: f64, t13824: f64) -> (f64, f64) {
    let t14443 = -t13771 + t13774 - t13776 + t13781 - t13783 + t13787 - t13793 - t13795 + t13799 - t13801 + t13803;
    let t14444 = t13806 + t13808 + t13810 - t13812 - t13816 - t13817 - t13818 + t13819 + t13820 + t13822 - t13823 + t13824;
    (t14443, t14444)
}
