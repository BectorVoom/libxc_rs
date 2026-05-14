//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1145/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1145<F: Float>(t11054: F, t1134: F, t1144: F, t12389: F, t1306: F, t135: F, t158: F, t26809: F, t273: F, t2957: F, t2965: F, t29753: F, t2990: F, t30193: F, t30195: F, t30197: F, t30200: F, t30203: F, t30205: F, t30208: F, t30211: F, t30807: F, t30982: F, t311: F, t3670: F, t3676: F, t3695: F, t800: F, t805: F, t9634: F, t9648: F, t9651: F, t9657: F) -> (F,) {
    let t30990 = -t29753 + t135 * t273 * (0.65854491829355115987e0 * t30807 * t158 * t311 - 0.65854491829355115987e0 * t11054 * t800 - 0.19756347548806534796e1 * t9634 * t1144 + 0.39512695097613069592e1 * t3670 * t2965 - 0.19756347548806534796e1 * t3670 * t2990 + 0.39512695097613069591e1 * t2957 * t3676 - 0.11853808529283920877e2 * t1134 * t9648 + 0.79025390195226139182e1 * t1134 * t9651 - 0.19756347548806534796e1 * t2957 * t3695 + 0.39512695097613069592e1 * t1134 * t9657 + t30982) * t805 - t30193 + t30195 - t30197 + 6.0 * t1306 * t26809 * t12389 + t30200 + t30203 - t30205 - t30208 - t30211;
    (t30990,)
}
