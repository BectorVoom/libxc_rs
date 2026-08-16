//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1004/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1004(t300: f64, t4715: f64, t4683: f64, t1626: f64, t983: f64, t1642: f64, t3022: f64, t1633: f64, t2986: f64, t974: f64, t981: f64, t4707: f64, t964: f64, t973: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4716 = t300 * t4715;
    let t4718 = 0.19751673498613801407e-1_f64 * t300 * t4683;
    let t4719 = t300 * t1626;
    let t4721 = 0.5848223622634646207e0_f64 * t4719 * t983;
    let t4723 = 0.5848223622634646207e0_f64 * t3022 * t1642;
    let t4724 = t2986 * t1633;
    let t4725 = t4724 * t974;
    let t4727 = 0.11696447245269292414e1_f64 * t981 * t4725;
    let t4729 = t964 * t4707 * t973;
    (t4716, t4718, t4719, t4721, t4723, t4724, t4725, t4727, t4729)
}
