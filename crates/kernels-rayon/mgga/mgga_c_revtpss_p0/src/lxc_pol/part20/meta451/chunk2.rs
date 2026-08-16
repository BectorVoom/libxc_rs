//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1719/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1719(t1408: f64, t2237: f64, t2482: f64, t3981: f64, t1412: f64, t3889: f64, t808: f64, t9736: f64, t1369: f64, t9726: f64, t1372: f64, t1410: f64, t1414: f64, t46345: f64, t46592: f64, t46596: f64, t46598: f64, t46600: f64, t46602: f64, t46607: f64, t46613: f64, t46618: f64, t46620: f64, t46622: f64, t46627: f64, t46628: f64, t46633: f64, t46641: f64, t828: f64) -> f64 {
    let t46644 = t2482 * t1408 * t2237;
    let t46645 = t46644 * t3981;
    let t46649 = t9736 * t808 * t1412 * t3889;
    let t46651 = t9726 * t1369;
    let t46652 = t46651 * t1372;
    let t46654 = -0.30492001685571196936e-3_f64 * t46592 + 0.6046824481244798459e0_f64 * t46596 - 0.45732285992607719437e-3_f64 * t46598 + 0.16006300097412701803e-1_f64 * t46600 + 0.32524801797942610064e-2_f64 * t46602 - 0.17149607247227894789e-2_f64 * t46607 + 0.17149607247227894789e-3_f64 * t46613 + 0.28582678745379824648e-4_f64 * t46618 + 0.28900264064772933811e-2_f64 * t46620 + 0.48018900292238105409e0_f64 * t46622 + 0.18007087609589289528e0_f64 * t1410 * t46627 * t828 * t46628 + 0.68026775414003982664e0_f64 * t46633 - 0.85748036236139473944e-3_f64 * t1410 * t1414 * t828 * t46345 - 0.2032800112371413129e-3_f64 * t46641 - 0.20553867802866510527e-1_f64 * t46645 + 0.6098400337114239387e-4_f64 * t46649 + 455.0_f64 / 162.0_f64 * t46652;
    t46654
}
