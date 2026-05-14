//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1031/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1031<F: Float>(t154: F, t18994: F, t385: F, t6406: F, t2387: F, t5728: F, t1478: F, t405: F, t824: F, t2185: F, t6446: F, t2380: F, t6475: F, t6479: F, t3185: F, t6412: F) -> (F, F, F, F, F, F, F) {
    let t18997 = t385 * t154 * t18994 * t6406;
    let t19014 = t2387 * t5728;
    let t19023 = t1478 * t405;
    let t19026 = t385 * t154 * t19023 * t824;
    let t19030 = t385 * t154 * t6446 * t2185;
    let t19033 = t2380 * t6475 * t6479;
    let t19036 = t3185 * t6475 * t6412;
    (t18997, t19014, t19023, t19026, t19030, t19033, t19036)
}
