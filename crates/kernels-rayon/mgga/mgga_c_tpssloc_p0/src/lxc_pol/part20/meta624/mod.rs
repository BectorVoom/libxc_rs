//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta624 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2246;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2247;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2248;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta624(t10109: f64, t1527: f64, t13036: f64, t225: f64, t2678: f64, t829: f64, t828: f64, t9632: f64, t1519: f64, t9971: f64, t13336: f64, t68: f64, t1496: f64, t41083: f64, t4257: f64, t9601: f64, t13193: f64, t2697: f64, t13204: f64, t2563: f64, t2379: f64, t40959: f64, t40962: f64, t40966: f64, t40982: f64, t40984: f64, t40988: f64, t40990: f64, t40998: f64, t4119: f64, t820: f64, t843: f64, t9607: f64, t842: f64, t4261: f64, t1516: f64, t40965: f64, t13347: f64, t119: f64, t13248: f64, t13254: f64, t13350: f64, t13365: f64, t210: f64, t2623: f64, t2643: f64, t2647: f64, t2703: f64, t40992: f64, t41009: f64, t41012: f64, t4172: f64, t46426: f64, t787: f64, t849: f64, t9609: f64, t9990: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46488, t46508, t46511, t46519, t46524, t46528) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2246(t10109, t1527, t13036, t225, t2678, t829, t828, t9632, t1519, t9971, t13336, t68);
        let t46560 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2247(t1496, t41083, t4257, t9601, t13193, t2697, t13204, t2563, t2379, t40959, t40962, t40966, t40982, t40984, t40988, t40990, t40998, t4119, t820, t843, t9607);
        let t46593 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2248(t4119, t828, t46528, t842, t4261, t9601, t1516, t40965, t13347, t2697, t119, t13248, t13254, t13350, t13365, t210, t2623, t2643, t2647, t2703, t40992, t41009, t41012, t4172, t46426, t787, t849, t9609, t9990);
    (t46488, t46508, t46511, t46519, t46524, t46528, t46560, t46593)
}
