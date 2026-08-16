//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1716/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1716(t12079: f64, t12122: f64, t12127: f64, t12168: f64, t16502: f64, t16552: f64, t16553: f64, t16559: f64, t16560: f64, t1668: f64, t19450: f64, t19603: f64, t23820: f64, t24083: f64, t24090: f64, t24135: f64, t24141: f64, t3304: f64, t3318: f64, t43520: f64, t43524: f64, t4893: f64, t4981: f64, t4982: f64, t55732: f64, t6299: f64, t80264: f64, t88794: f64, t88804: f64) -> f64 {
    let t89603 = -0.15805078039045227836e2_f64 * t43520 * t88794 * t12168 + 0.15805078039045227836e2_f64 * t43524 * t88794 * t12079 + 0.79025390195226139183e1_f64 * t55732 * t24141 - 0.79025390195226139183e1_f64 * t12122 * t88804 * t3304 + 0.39512695097613069592e1_f64 * t12127 * t88804 * t3318 - 0.79025390195226139183e1_f64 * t16502 * t24135 + 0.15805078039045227836e2_f64 * t19603 * t24090 + 0.52683593463484092788e1_f64 * t4981 * t4893 * t4982 * t23820 + 0.23707617058567841754e2_f64 * t16552 * t19450 * t16553 * t6299 - 0.23707617058567841754e2_f64 * t16559 * t19450 * t16560 * t6299 - 0.15805078039045227836e2_f64 * t12122 * t80264 * t4982 * t1668 + 0.79025390195226139184e1_f64 * t12127 * t80264 * t24083;
    t89603
}
