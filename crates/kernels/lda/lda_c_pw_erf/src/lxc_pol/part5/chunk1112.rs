//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1112/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1112<F: Float>(t34: F, t6723: F, t13771: F, t4522: F, t17645: F, t2034: F, t3974: F, t16612: F, t2010: F, t4506: F, t20688: F, t20689: F, t20691: F, t20693: F, t20695: F, t20700: F, t20704: F, t20707: F, t20710: F, t20715: F) -> (F, F, F, F, F) {
    let t20716 = t6723 * t34;
    let t20719 = F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t13771 * t4522 * t20716;
    let t20722 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t3974 * t17645 * t2034;
    let t20725 = F::cast_from(8.0_f64) / F::cast_from(5.0_f64) * t4506 * t16612 * t2010;
    let t20726 = t20688 + t20689 - t20691 - t20693 + t20695 - t20700 - t20704 + t20707 - t20710 - t20715 + t20719 + t20722 - t20725;
    (t20716, t20719, t20722, t20725, t20726)
}
