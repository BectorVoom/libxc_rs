//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1290/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1290<F: Float>(t4804: F, t7601: F, t3794: F, t18673: F, t18695: F, t565: F, t7458: F, t211: F, t514: F, t7457: F, t18710: F, t18712: F) -> (F, F, F, F, F, F, F, F) {
    let t23067 = F::new(4.0) / F::new(5.0) * t4804 * t7601;
    let t23069 = F::new(4.0) / F::new(5.0) * t3794 * t7601;
    let t23070 = F::new(16.0) / F::new(45.0) * t18673;
    let t23071 = F::new(8.0) / F::new(15.0) * t18695;
    let t23073 = F::new(2.0) / F::new(15.0) * t565 * t7458;
    let t23075 = t211 * t514 * t7457;
    let t23076 = F::new(4.0) / F::new(45.0) * t23075;
    let t23077 = F::new(4.0) / F::new(15.0) * t18710;
    let t23078 = F::new(8.0) / F::new(15.0) * t18712;
    (t23067, t23069, t23070, t23071, t23073, t23076, t23077, t23078)
}
