//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1171/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1171<F: Float>(t34: F, t352: F, t593: F, t13771: F, t4522: F, t1287: F, t743: F, t3974: F, t5160: F, t13107: F, t11907: F, t13111: F) -> (F, F, F, F, F, F, F) {
    let t13773 = t34 * t593 * t352;
    let t13776 = F::new(16.0) / F::new(9.0) * t13771 * t4522 * t13773;
    let t13777 = t743 * t1287;
    let t13778 = t13777 * t352;
    let t13781 = F::new(16.0) / F::new(15.0) * t3974 * t5160 * t13778;
    let t13784 = F::new(16.0) / F::new(15.0) * t3974 * t5160 * t13107;
    let t13787 = F::new(16.0) / F::new(3.0) * t3974 * t11907 * t13111;
    (t13773, t13776, t13777, t13778, t13781, t13784, t13787)
}
