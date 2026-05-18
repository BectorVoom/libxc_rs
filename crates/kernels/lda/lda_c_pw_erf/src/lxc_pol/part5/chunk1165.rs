//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1165/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1165<F: Float>(t16955: F, t16957: F, t16961: F, t16963: F, t9593: F, t16989: F, t2134: F, t2443: F, t12572: F, t12616: F, t6297: F, t822: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t21322 = F::new(16.0) / F::new(45.0) * t16955;
    let t21323 = F::new(4.0) / F::new(15.0) * t16957;
    let t21324 = F::new(8.0) / F::new(45.0) * t16961;
    let t21325 = F::new(4.0) / F::new(15.0) * t16963;
    let t21326 = F::new(32.0) / F::new(405.0) * t9593;
    let t21327 = F::new(8.0) / F::new(15.0) * t16989;
    let t21328 = t2443 * t2134;
    let t21329 = F::new(4.0) / F::new(15.0) * t21328;
    let t21330 = F::new(32.0) / F::new(405.0) * t12572;
    let t21331 = F::new(32.0) / F::new(405.0) * t12616;
    let t21332 = t822 * t6297;
    (t21322, t21323, t21324, t21325, t21326, t21327, t21329, t21330, t21331, t21332)
}
