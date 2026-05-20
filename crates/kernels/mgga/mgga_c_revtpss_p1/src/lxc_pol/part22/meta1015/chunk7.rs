//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3507/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3507<F: Float>(t19912: F, t3241: F, t1011: F, t6292: F, t697: F, t11922: F, t19717: F, t4899: F, t11883: F, t16147: F, t19705: F, t3092: F, t53948: F, t53955: F, t53958: F, t53961: F, t53964: F, t53967: F, t53970: F, t53974: F, t55331: F, t6293: F) -> F {
    let t66215 = t3241 * t19912;
    let t66218 = t1011 * t697 * t6292;
    let t66221 = t4899 * t11922 * t19717;
    let t66227 = -F::cast_from(0.30488190661738479624e-2_f64) * t53948 - F::cast_from(0.1270341277572436651e-3_f64) * t53955 - t53958 / F::new(108.0) - t53961 / F::new(216.0) - t53964 / F::new(54.0) + t53967 / F::new(162.0) + t53970 / F::new(324.0) + F::new(7.0) / F::new(972.0) * t53974 + F::new(11.0) / F::new(243.0) * t11883 * t6293 - F::new(2.0) / F::new(243.0) * t66215 - t66218 / F::new(972.0) - F::cast_from(0.57165357490759649296e-3_f64) * t66221 - F::cast_from(0.17149607247227894789e-2_f64) * t55331 * t3092 * t19705 * t16147;
    t66227
}
