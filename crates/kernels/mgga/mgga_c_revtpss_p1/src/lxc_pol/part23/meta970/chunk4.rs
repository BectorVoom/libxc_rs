//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3274/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3274<F: Float>(t22914: F, t3930: F, t124: F, t1353: F, t1370: F, t13783: F, t1388: F, t1390: F, t1410: F, t22813: F, t3934: F, t46627: F, t46831: F, t46833: F, t46840: F, t46859: F, t48756: F, t5627: F, t6844: F, t74402: F, t74421: F, t800: F, t828: F, t85442: F, t85873: F, t85885: F, t86054: F, t86061: F, t86070: F, t86074: F, t86078: F) -> F {
    let t86080 = t3930 * t22914;
    let t86086 = F::cast_from(0.85748036236139473944e-4_f64) * t74402 + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t85873 - t1370 * t800 * t124 * t85442 / F::cast_from(48.0_f64) - F::cast_from(0.12862205435420921092e-1_f64) * t3934 * t13783 * t6844 * t5627 - F::cast_from(0.42874018118069736973e-4_f64) * t85885 - F::cast_from(0.21437009059034868486e-3_f64) * t1388 * t1390 * t828 * t86054 - F::cast_from(0.12705000702321332056e-4_f64) * t86061 + F::cast_from(0.18007087609589289528e0_f64) * t1410 * t46627 * t828 * t22813 * t1353 + F::cast_from(0.42874018118069736973e-3_f64) * t86070 - F::cast_from(0.7623000421392799234e-4_f64) * t86074 + F::cast_from(0.7623000421392799234e-4_f64) * t86078 + F::cast_from(0.10003937560882938627e-2_f64) * t86080 - t46831 + F::cast_from(0.68026775414003982663e-1_f64) * t48756 + F::cast_from(0.81322168495418382223e-4_f64) * t46833 + t46840 - F::cast_from(0.20082057720118594944e-6_f64) * t46859 - F::cast_from(0.45738002528356795403e-2_f64) * t74421;
    t86086
}
