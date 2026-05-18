//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1145/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1145<F: Float>(t5986: F, t7822: F, t5981: F, t1881: F, t7605: F, t142: F, t6319: F, t8888: F, t2060: F, t6293: F, t7815: F, t2288: F, t8901: F) -> (F, F, F, F, F, F) {
    let t39815 = t7822 * t5986;
    let t39817 = t7822 * t5981;
    let t39819 = t7605 * t1881;
    let t39822 = t8888 * t142 * t6319;
    let t39825 = t2060 * t7815 * t6293;
    let t39827 = t2288 * t8901;
    (t39815, t39817, t39819, t39822, t39825, t39827)
}
