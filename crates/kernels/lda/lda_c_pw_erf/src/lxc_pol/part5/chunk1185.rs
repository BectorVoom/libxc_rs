//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1185/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1185<F: Float>(t15685: F, t6981: F, t581: F, t7836: F, t1318: F, t1466: F, t593: F, t4738: F, t6999: F, t17396: F, t17398: F, t17413: F) -> (F, F, F, F, F, F, F) {
    let t21544 = F::new(4.0) / F::new(5.0) * t15685 * t6981;
    let t21545 = t581 * t7836;
    let t21549 = F::new(4.0) / F::new(15.0) * t1318 * t1466 * t21545 * t593;
    let t21551 = F::new(4.0) / F::new(5.0) * t4738 * t6999;
    let t21553 = F::new(4.0) / F::new(5.0) * t4738 * t6981;
    let t21554 = F::new(32.0) / F::new(45.0) * t17396;
    let t21555 = F::new(64.0) / F::new(45.0) * t17398;
    let t21556 = F::new(4.0) / F::new(15.0) * t17413;
    (t21544, t21549, t21551, t21553, t21554, t21555, t21556)
}
