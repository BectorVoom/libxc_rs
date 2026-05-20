//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1155/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1155<F: Float>(t16088: F, t16094: F, t3169: F, t4820: F, t3188: F, t4817: F, t1065: F, t4772: F, t247: F, t3109: F, t4583: F, t1063: F) -> (F, F, F, F, F) {
    let t16095 = t16094 * t16088;
    let t16121 = F::cast_from(0.15244095330869239812e-2_f64) * t3169 * t4820;
    let t16134 = F::cast_from(0.19055119163586549765e-3_f64) * t3188 * t4817;
    let t16138 = t1065 * t4772;
    let t16158 = t247 * t3109 * t4583;
    let t16160 = F::cast_from(0.19055119163586549765e-3_f64) * t1063 * t16158;
    (t16095, t16121, t16134, t16138, t16160)
}
