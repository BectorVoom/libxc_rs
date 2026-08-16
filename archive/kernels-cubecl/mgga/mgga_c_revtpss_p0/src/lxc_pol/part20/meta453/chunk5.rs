//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1734/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1734<F: Float>(t2689: F, t9715: F, t40688: F, t547: F, t46786: F, t807: F, t9400: F, t9941: F, t9704: F, t4003: F, t46531: F, t124: F, t1370: F, t13789: F, t13791: F, t1390: F, t1410: F, t3829: F, t3889: F, t3934: F, t3938: F, t4002: F, t46345: F, t46432: F, t46853: F, t46859: F, t46861: F, t46863: F, t46865: F, t46877: F, t46879: F, t46885: F, t5671: F, t800: F, t828: F, t9840: F, t9942: F) -> (F, F) {
    let t46886 = t2689 * t9715;
    let t46888 = t40688 * t547;
    let t46889 = t46888 * t46786;
    let t46893 = t807 * t547 * t9941 * t9400;
    let t46895 = t2689 * t9704;
    let t46902 = t46531 * t4003;
    let t46911 = F::cast_from(0.30492001685571196935e-3_f64) * t46853 - F::cast_from(0.80328230880474379775e-6_f64) * t46859 + F::cast_from(0.81312004494856525159e-3_f64) * t46861 + F::cast_from(0.40015750243531754508e-2_f64) * t46863 + F::cast_from(7.0_f64) / F::cast_from(36.0_f64) * t46865 - t1370 * t800 * t124 * t46345 / F::cast_from(48.0_f64) + F::cast_from(0.10289764348336736873e-1_f64) * t3934 * t13789 * t46432 * t3938 + F::cast_from(0.11433071498151929859e-3_f64) * t46877 + F::cast_from(0.11560105625909173524e-1_f64) * t46879 + t46885 - F::cast_from(0.18292914397043087775e-2_f64) * t46886 + F::cast_from(0.18071592998981862717e-5_f64) * t46889 + F::cast_from(0.34299214494455789577e-2_f64) * t46893 + F::cast_from(0.91464571985215438873e-2_f64) * t46895 - F::cast_from(0.1543464652250510531e0_f64) * t1410 * t9942 * t828 * t3889 * t3829 + F::cast_from(0.12862205435420921092e-2_f64) * t4002 * t1390 * t828 * t46902 - F::cast_from(0.20579528696673473747e-1_f64) * t5671 * t13789 * t9840 * t13791;
    (t46902, t46911)
}
