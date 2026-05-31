//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1719/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1719<F: Float>(t1408: F, t2237: F, t2482: F, t3981: F, t1412: F, t3889: F, t808: F, t9736: F, t1369: F, t9726: F, t1372: F, t1410: F, t1414: F, t46345: F, t46592: F, t46596: F, t46598: F, t46600: F, t46602: F, t46607: F, t46613: F, t46618: F, t46620: F, t46622: F, t46627: F, t46628: F, t46633: F, t46641: F, t828: F) -> F {
    let t46644 = t2482 * t1408 * t2237;
    let t46645 = t46644 * t3981;
    let t46649 = t9736 * t808 * t1412 * t3889;
    let t46651 = t9726 * t1369;
    let t46652 = t46651 * t1372;
    let t46654 = -F::cast_from(0.30492001685571196936e-3_f64) * t46592 + F::cast_from(0.6046824481244798459e0_f64) * t46596 - F::cast_from(0.45732285992607719437e-3_f64) * t46598 + F::cast_from(0.16006300097412701803e-1_f64) * t46600 + F::cast_from(0.32524801797942610064e-2_f64) * t46602 - F::cast_from(0.17149607247227894789e-2_f64) * t46607 + F::cast_from(0.17149607247227894789e-3_f64) * t46613 + F::cast_from(0.28582678745379824648e-4_f64) * t46618 + F::cast_from(0.28900264064772933811e-2_f64) * t46620 + F::cast_from(0.48018900292238105409e0_f64) * t46622 + F::cast_from(0.18007087609589289528e0_f64) * t1410 * t46627 * t828 * t46628 + F::cast_from(0.68026775414003982664e0_f64) * t46633 - F::cast_from(0.85748036236139473944e-3_f64) * t1410 * t1414 * t828 * t46345 - F::cast_from(0.2032800112371413129e-3_f64) * t46641 - F::cast_from(0.20553867802866510527e-1_f64) * t46645 + F::cast_from(0.6098400337114239387e-4_f64) * t46649 + F::cast_from(455.0_f64) / F::cast_from(162.0_f64) * t46652;
    t46654
}
