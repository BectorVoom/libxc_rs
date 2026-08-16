//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3910/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3910<F: Float>(t10073: F, t22361: F, t10069: F, t22373: F, t10139: F, t136: F, t2457: F, t6874: F, t1399: F, t14255: F, t46536: F, t46542: F, t49198: F, t49200: F, t49203: F, t49208: F, t49210: F, t5659: F, t5675: F, t5745: F, t5755: F, t74965: F, t74982: F, t820: F) -> F {
    let t75113 = t10073 * t22361;
    let t75119 = t10069 * t22373;
    let t75123 = t10139 * t6874 * t136 * t2457;
    let t75125 = -F::cast_from(0.13170898365871023197e1_f64) * t5755 * t74982 * t1399 - F::cast_from(0.26341796731742046394e1_f64) * t820 * t14255 * t5659 + F::cast_from(0.52039682876708176102e-1_f64) * t49198 - F::cast_from(0.19514881078765566038e-1_f64) * t49200 - F::cast_from(0.60712963356159538786e-1_f64) * t49203 + F::cast_from(0.21951497276451705328e-1_f64) * t49208 - F::cast_from(0.520396828767081761e-2_f64) * t49210 - F::cast_from(0.73171657588172351096e-2_f64) * t46536 - F::cast_from(0.13009920719177044025e-2_f64) * t75113 + F::cast_from(0.26341796731742046394e1_f64) * t5745 * t74965 * t5675 - F::cast_from(0.73171657588172351096e-2_f64) * t46542 - F::cast_from(0.73171657588172351096e-2_f64) * t75119 - F::cast_from(0.11565819519348392139e-2_f64) * t75123;
    t75125
}
