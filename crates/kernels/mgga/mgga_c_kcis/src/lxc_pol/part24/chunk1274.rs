//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1274/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1274<F: Float>(t28093: F, t28204: F, t19684: F, t303: F, t356: F, t1014: F, t29000: F, t1856: F, t829: F, t4580: F, t96935: F, t4566: F, t96793: F) -> (F, F, F, F, F) {
    let t100834 = t28204 * t28093;
    let t100841 = t303 * t356 * t19684;
    let t100843 = t1014 * t29000;
    let t100845 = t1856 * t829;
    let t100847 = t96935 * t4580 * t100845;
    let t100851 = t96793 * t4566 * t100845;
    (t100834, t100841, t100843, t100847, t100851)
}
