//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1734/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1734(t2689: f64, t9715: f64, t40688: f64, t547: f64, t46786: f64, t807: f64, t9400: f64, t9941: f64, t9704: f64, t4003: f64, t46531: f64, t124: f64, t1370: f64, t13789: f64, t13791: f64, t1390: f64, t1410: f64, t3829: f64, t3889: f64, t3934: f64, t3938: f64, t4002: f64, t46345: f64, t46432: f64, t46853: f64, t46859: f64, t46861: f64, t46863: f64, t46865: f64, t46877: f64, t46879: f64, t46885: f64, t5671: f64, t800: f64, t828: f64, t9840: f64, t9942: f64) -> (f64, f64) {
    let t46886 = t2689 * t9715;
    let t46888 = t40688 * t547;
    let t46889 = t46888 * t46786;
    let t46893 = t807 * t547 * t9941 * t9400;
    let t46895 = t2689 * t9704;
    let t46902 = t46531 * t4003;
    let t46911 = 0.30492001685571196935e-3_f64 * t46853 - 0.80328230880474379775e-6_f64 * t46859 + 0.81312004494856525159e-3_f64 * t46861 + 0.40015750243531754508e-2_f64 * t46863 + 7.0_f64 / 36.0_f64 * t46865 - t1370 * t800 * t124 * t46345 / 48.0_f64 + 0.10289764348336736873e-1_f64 * t3934 * t13789 * t46432 * t3938 + 0.11433071498151929859e-3_f64 * t46877 + 0.11560105625909173524e-1_f64 * t46879 + t46885 - 0.18292914397043087775e-2_f64 * t46886 + 0.18071592998981862717e-5_f64 * t46889 + 0.34299214494455789577e-2_f64 * t46893 + 0.91464571985215438873e-2_f64 * t46895 - 0.1543464652250510531e0_f64 * t1410 * t9942 * t828 * t3889 * t3829 + 0.12862205435420921092e-2_f64 * t4002 * t1390 * t828 * t46902 - 0.20579528696673473747e-1_f64 * t5671 * t13789 * t9840 * t13791;
    (t46902, t46911)
}
