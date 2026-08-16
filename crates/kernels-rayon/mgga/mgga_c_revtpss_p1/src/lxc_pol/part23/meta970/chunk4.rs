//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3274/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3274(t22914: f64, t3930: f64, t124: f64, t1353: f64, t1370: f64, t13783: f64, t1388: f64, t1390: f64, t1410: f64, t22813: f64, t3934: f64, t46627: f64, t46831: f64, t46833: f64, t46840: f64, t46859: f64, t48756: f64, t5627: f64, t6844: f64, t74402: f64, t74421: f64, t800: f64, t828: f64, t85442: f64, t85873: f64, t85885: f64, t86054: f64, t86061: f64, t86070: f64, t86074: f64, t86078: f64) -> f64 {
    let t86080 = t3930 * t22914;
    let t86086 = 0.85748036236139473944e-4_f64 * t74402 + 7.0_f64 / 144.0_f64 * t85873 - t1370 * t800 * t124 * t85442 / 48.0_f64 - 0.12862205435420921092e-1_f64 * t3934 * t13783 * t6844 * t5627 - 0.42874018118069736973e-4_f64 * t85885 - 0.21437009059034868486e-3_f64 * t1388 * t1390 * t828 * t86054 - 0.12705000702321332056e-4_f64 * t86061 + 0.18007087609589289528e0_f64 * t1410 * t46627 * t828 * t22813 * t1353 + 0.42874018118069736973e-3_f64 * t86070 - 0.7623000421392799234e-4_f64 * t86074 + 0.7623000421392799234e-4_f64 * t86078 + 0.10003937560882938627e-2_f64 * t86080 - t46831 + 0.68026775414003982663e-1_f64 * t48756 + 0.81322168495418382223e-4_f64 * t46833 + t46840 - 0.20082057720118594944e-6_f64 * t46859 - 0.45738002528356795403e-2_f64 * t74421;
    t86086
}
