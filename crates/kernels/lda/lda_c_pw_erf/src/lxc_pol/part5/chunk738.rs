//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 738/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk738<F: Float>(t203: F, t6669: F, t184: F, t221: F, t3910: F, t3917: F, t3919: F, t3923: F, t3929: F, t3935: F, t3938: F, t3950: F, t3951: F, t4595: F, t6599: F, t6603: F, t6605: F, t6606: F, t6613: F, t6633: F) -> (F, F, F, F) {
    let t6670 = t203 * t6669;
    let t6671 = t6670 * t184;
    let t6673 = F::new(2.0) / F::new(15.0) * t6671 * t221;
    let t6674 = t6599 + t6603 + t6605 - t6606 - t4595 + F::new(4.0) / F::new(9.0) * t3910 + t3917 + t3919 + t3923 + t3929 + t3935 - t3938 + t3950 / F::new(3.0) + F::new(0.12155555555555556) * t3951 + t6613 + t6633 + t6673;
    (t6670, t6671, t6673, t6674)
}
