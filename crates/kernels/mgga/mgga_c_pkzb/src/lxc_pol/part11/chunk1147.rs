//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1147/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1147<F: Float>(t10282: F, t914: F, t10334: F, t6455: F, t10066: F, t3206: F, t6475: F, t10195: F, t178: F, t915: F, t10050: F, t2380: F) -> (F, F, F, F, F) {
    let t26927 = t914 * t10282;
    let t26948 = t6455 * t10334;
    let t26970 = t3206 * t6475 * t10066;
    let t26975 = t915 * t10195 * t178;
    let t26981 = t2380 * t6475 * t10050;
    (t26927, t26948, t26970, t26975, t26981)
}
