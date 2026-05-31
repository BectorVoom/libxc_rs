//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 814/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk814<F: Float>(t1684: F, t3031: F, t1823: F, t3549: F, t110: F, t1852: F, t1251: F, t3490: F, t5321: F, t25: F, t5337: F, t11081: F, t5325: F) -> (F, F, F, F, F, F) {
    let t15450 = t1684 * t3031;
    let t15460 = t1823 * t3549;
    let t15476 = t110 * t1852;
    let t15477 = t1251 * t15476;
    let t15493 = t3490 * t5321 / F::cast_from(108.0_f64);
    let t15494 = t25 * t5337;
    let t15496 = t1251 * t15494 / F::cast_from(288.0_f64);
    let t15516 = t11081 * t5325;
    (t15450, t15460, t15477, t15493, t15496, t15516)
}
