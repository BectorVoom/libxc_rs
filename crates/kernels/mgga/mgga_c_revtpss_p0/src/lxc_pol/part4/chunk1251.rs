//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1251/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1251<F: Float>(t15525: F, t4733: F, t981: F, t15495: F, t300: F, t15234: F, t964: F, t973: F, t2986: F, t4707: F, t974: F, t11506: F, t1633: F) -> (F, F, F, F, F) {
    let t15526 = t15525 * t4733;
    let t15528 = F::cast_from(0.34631718211362927518e2_f64) * t981 * t15526;
    let t15530 = F::cast_from(0.19751673498613801407e-1_f64) * t300 * t15495;
    let t15534 = t964 * t15234 * t973;
    let t15536 = F::cast_from(0.5848223622634646207e0_f64) * t981 * t15534;
    let t15537 = t2986 * t4707;
    let t15538 = t15537 * t974;
    let t15540 = F::cast_from(0.23392894490538584828e1_f64) * t981 * t15538;
    let t15541 = t11506 * t1633;
    (t15528, t15530, t15536, t15540, t15541)
}
