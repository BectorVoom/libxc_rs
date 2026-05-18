//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 654/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk654<F: Float>(t1426: F, t245: F, t645: F, t1433: F, t656: F, t1: F, t1578: F, t119: F, t646: F, t1423: F, t3862: F, t3866: F, t3871: F, t3875: F, t3877: F, t3879: F, t3882: F, t3886: F, t3890: F, t3898: F, t3902: F, t3907: F, t3908: F, t3910: F, t3912: F) -> (F, F, F, F, F, F, F) {
    let t3915 = t245 * t1426;
    let t3917 = 2e-21 * t645 * t3915;
    let t3919 = F::new(2.0) / F::new(3.0) * t1433 * t656;
    let t3920 = t1578 * t1;
    let t3921 = t119 * t646;
    let t3923 = F::new(0.001515438175925926) * t3920 * t3921;
    let t3924 = t3862 - t3866 + t3871 + t3875 + t3877 + t3879 + t3882 + t3886 + t3890 + t3898 - t3902 - t3907 + F::new(2.0) / F::new(3.0) * t3908 + F::new(4.0) / F::new(3.0) * t3910 + 2e-21 * t1423 * t3912 + t3917 + t3919 + t3923;
    (t3915, t3917, t3919, t3920, t3921, t3923, t3924)
}
