//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1030/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1030<F: Float>(t3206: F, t6372: F, t6475: F, t2370: F, t6506: F, t2368: F, t5728: F, t154: F, t2347: F, t385: F, t6106: F, t486: F, t931: F, t2226: F, t2411: F, t67: F) -> (F, F, F, F, F, F) {
    let t18967 = t3206 * t6475 * t6372;
    let t18974 = t2370 * t6506;
    let t18979 = t2368 * t5728;
    let t18987 = t385 * t154 * t2347 * t6106;
    let t18989 = t486 * t931;
    let t18992 = t385 * t154 * t18989 * t2226;
    let t18994 = t67 * t2411;
    (t18967, t18974, t18979, t18987, t18992, t18994)
}
