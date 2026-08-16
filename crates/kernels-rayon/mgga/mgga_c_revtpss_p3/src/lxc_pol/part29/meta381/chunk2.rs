//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1364/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1364(t5710: f64, t72: f64, t1432: f64, t686: f64, t136: f64, t1892: f64, t2457: f64, t3964: f64, t2435: f64, t5760: f64, t3999: f64, t545: f64) -> (f64, f64, f64, f64, f64) {
    let t14155 = t5710 * t72;
    let t14158 = 0.19514881078765566038e-1_f64 * t1432 * t14155 * t686;
    let t14159 = t1892 * t136;
    let t14161 = t3964 * t14159 * t2457;
    let t14166 = t2435 * t5760;
    let t14171 = t3999 * t1892;
    let t14188 = t545 * t5710;
    (t14158, t14161, t14166, t14171, t14188)
}
