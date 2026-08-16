//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2871/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2871<F: Float>(t14546: F, t1559: F, t18677: F, t40922: F, t4514: F, t51578: F, t51588: F, t51604: F, t51615: F, t62612: F, t62952: F, t62961: F, t62968: F, t76726: F, t77120: F, t820: F, t879: F) -> F {
    let t77259 = -F::cast_from(0.33133632253434461091e-3_f64) * t51578 - t51588 + F::cast_from(0.17073386770573548589e-1_f64) * t40922 - F::cast_from(0.65854491829355115987e0_f64) * t820 * t879 * t77120 + F::cast_from(0.39029762157531132076e-1_f64) * t62952 - t51604 - F::cast_from(0.32927245914677557992e-1_f64) * t62961 - t51615 - F::cast_from(0.29272321618148349057e-1_f64) * t62968 - F::cast_from(0.11853808529283920877e2_f64) * t14546 * t18677 * t76726 - F::cast_from(0.19756347548806534796e1_f64) * t4514 * t62612 * t1559;
    t77259
}
