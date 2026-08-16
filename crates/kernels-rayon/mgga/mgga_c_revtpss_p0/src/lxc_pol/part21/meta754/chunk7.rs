//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2645/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2645(t48668: f64, t13783: f64, t1410: f64, t1868: f64, t1883: f64, t3934: f64, t3936: f64, t46627: f64, t46754: f64, t46757: f64, t46760: f64, t46767: f64, t46771: f64, t46776: f64, t46780: f64, t46787: f64, t46789: f64, t46793: f64, t46797: f64, t48655: f64, t48664: f64, t48666: f64, t5704: f64, t828: f64, t9400: f64, t9891: f64, t9984: f64) -> f64 {
    let t48669 = 0.40656002247428262579e-3_f64 * t48668;
    let t48683 = 0.85748036236139473944e-3_f64 * t3934 * t3936 * t5704 * t9891 + 0.15246000842785598467e-3_f64 * t46754 + 0.15246000842785598468e-3_f64 * t48655 - 0.12862205435420921092e-1_f64 * t3934 * t13783 * t1883 * t9984 - 0.13553694749236397037e-4_f64 * t46757 - t46760 - 0.45738002528356795402e-2_f64 * t48664 + 0.30011812682648815881e-2_f64 * t48666 + t48669 + 0.18007087609589289528e0_f64 * t1410 * t46627 * t828 * t1868 * t9400 + 0.34013387707001991333e-1_f64 * t46767 + 0.21437009059034868486e-4_f64 * t46771 - 0.85748036236139473944e-4_f64 * t46776 - 0.85748036236139473944e-4_f64 * t46780 - 0.24098469264142313933e-5_f64 * t46787 - 0.22866142996303859718e-3_f64 * t46789 + 0.71456696863449561619e-5_f64 * t46793 - 0.42874018118069736972e-4_f64 * t46797;
    t48683
}
