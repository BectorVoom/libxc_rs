//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1178/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1178<F: Float>(t179: F, t19155: F, t2226: F, t404: F, t154: F, t385: F, t386: F, t4932: F, t2370: F, t466: F, t931: F, t2380: F, t2383: F, t414: F, t6545: F, t18980: F, t7832: F) -> (F, F, F, F, F, F, F) {
    let t19158 = t404 * t179 * t19155 * t2226;
    let t19163 = 5.0 / 486.0 * t385 * t154 * t4932 * t386;
    let t19182 = t2370 * t2226;
    let t19191 = t466 * t931;
    let t19193 = t2380 * t19191 * t2383;
    let t19227 = 1.0 / t6545 / t414;
    let t19245 = t7832 * t18980;
    (t19158, t19163, t19182, t19191, t19193, t19227, t19245)
}
