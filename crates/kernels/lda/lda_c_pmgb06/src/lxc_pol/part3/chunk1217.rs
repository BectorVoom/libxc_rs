//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1217/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1217<F: Float>(t13771: F, t13774: F, t13776: F, t13781: F, t13783: F, t13787: F, t13793: F, t13795: F, t13799: F, t13801: F, t13803: F, t13806: F, t13808: F, t13810: F, t13812: F, t13816: F, t13817: F, t13818: F, t13819: F, t13820: F, t13822: F, t13823: F, t13824: F) -> (F, F) {
    let t14443 = -t13771 + t13774 - t13776 + t13781 - t13783 + t13787 - t13793 - t13795 + t13799 - t13801 + t13803;
    let t14444 = t13806 + t13808 + t13810 - t13812 - t13816 - t13817 - t13818 + t13819 + t13820 + t13822 - t13823 + t13824;
    (t14443, t14444)
}
