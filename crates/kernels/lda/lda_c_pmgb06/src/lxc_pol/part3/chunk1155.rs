//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1155/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1155<F: Float>(t1966: F, t3031: F, t1629: F, t439: F, t477: F, t822: F, t1916: F, t3198: F, t13762: F, t13764: F, t13767: F, t13769: F, t13771: F, t13774: F, t13776: F, t13781: F, t13783: F, t13787: F) -> (F, F, F) {
    let t13788 = t1966 * t3031;
    let t13793 = F::new(3.0) / F::new(5.0) * t439 * t13788 * t822 * t1629 * t477;
    let t13795 = F::new(2.0) / F::new(15.0) * t3198 * t1916;
    let t13796 = t13762 + t13764 + t13767 - t13769 - t13771 + t13774 - t13776 + t13781 - t13783 + t13787 - t13793 - t13795;
    (t13793, t13795, t13796)
}
