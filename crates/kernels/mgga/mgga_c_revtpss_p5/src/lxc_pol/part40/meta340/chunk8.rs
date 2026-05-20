//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1149/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1149<F: Float>(t13564: F, t13579: F, t162: F, t187: F, t1857: F, t3857: F, t5591: F, t566: F, t9375: F, t177: F, t5566: F, t762: F) -> (F, F, F, F, F, F) {
    let t13581 = (t13564 + t13579) * t162;
    let t13583 = F::cast_from(0.19751673498613801407e-1_f64) * t13581 * t187;
    let t13584 = t3857 * t1857;
    let t13585 = F::new(20.0) * t13584;
    let t13586 = t566 * t5591;
    let t13593 = F::cast_from(0.11696447245269292414e1_f64) * t9375;
    let t13597 = t5566 * t177;
    let t13599 = F::cast_from(0.11696447245269292414e1_f64) * t13597 * t762;
    (t13581, t13583, t13585, t13586, t13593, t13599)
}
