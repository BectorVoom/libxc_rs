//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3293/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3293(t22953: f64, t555: f64, t22954: f64, t4101: f64, t686: f64, t72: f64, t1892: f64, t6861: f64, t14193: f64, t1883: f64, t21990: f64, t22005: f64, t22016: f64, t46515: f64, t46518: f64, t48080: f64, t48082: f64, t48090: f64, t5675: f64, t5745: f64, t5755: f64, t74965: f64, t75060: f64) -> (f64, f64, f64) {
    let t86455 = t555 * t22953;
    let t86468 = t4101 * t22954 * t72 * t686;
    let t86470 = t1892 * t6861;
    let t86474 = -t46515 + 0.13170898365871023197e1_f64 * t5745 * t86455 * t5675 - 0.19756347548806534796e1_f64 * t5755 * t74965 * t1883 + t48080 + t48082 + 0.11853808529283920877e2_f64 * t5745 * t22005 * t21990 + t48090 + 0.58544643236296698113e-1_f64 * t75060 - 0.9757440539382783019e-2_f64 * t86468 + t46518 - 0.11853808529283920877e2_f64 * t14193 * t86470 * t22016;
    (t86455, t86470, t86474)
}
