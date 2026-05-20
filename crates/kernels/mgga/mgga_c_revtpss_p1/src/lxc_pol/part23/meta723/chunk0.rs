//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2486/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2486<F: Float>(t1389: F, t14230: F, t2735: F, t46801: F, t40763: F, t5609: F, t9793: F, t13830: F, t9775: F, t13760: F, t9765: F, t268: F, t5617: F) -> (F, F, F, F, F) {
    let t48876 = t2735 * t46801 * t1389 * t14230;
    let t48877 = F::cast_from(0.15246000842785598467e-4_f64) * t48876;
    let t48879 = t9793 * t40763 * t5609;
    let t48881 = t9775 * t13830;
    let t48904 = t9765 * t13760;
    let t48905 = F::cast_from(0.16262400898971305032e-2_f64) * t48904;
    let t48908 = t5617 * t268;
    (t48877, t48879, t48881, t48905, t48908)
}
