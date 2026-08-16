//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2645/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2645<F: Float>(t48668: F, t13783: F, t1410: F, t1868: F, t1883: F, t3934: F, t3936: F, t46627: F, t46754: F, t46757: F, t46760: F, t46767: F, t46771: F, t46776: F, t46780: F, t46787: F, t46789: F, t46793: F, t46797: F, t48655: F, t48664: F, t48666: F, t5704: F, t828: F, t9400: F, t9891: F, t9984: F) -> F {
    let t48669 = F::cast_from(0.40656002247428262579e-3_f64) * t48668;
    let t48683 = F::cast_from(0.85748036236139473944e-3_f64) * t3934 * t3936 * t5704 * t9891 + F::cast_from(0.15246000842785598467e-3_f64) * t46754 + F::cast_from(0.15246000842785598468e-3_f64) * t48655 - F::cast_from(0.12862205435420921092e-1_f64) * t3934 * t13783 * t1883 * t9984 - F::cast_from(0.13553694749236397037e-4_f64) * t46757 - t46760 - F::cast_from(0.45738002528356795402e-2_f64) * t48664 + F::cast_from(0.30011812682648815881e-2_f64) * t48666 + t48669 + F::cast_from(0.18007087609589289528e0_f64) * t1410 * t46627 * t828 * t1868 * t9400 + F::cast_from(0.34013387707001991333e-1_f64) * t46767 + F::cast_from(0.21437009059034868486e-4_f64) * t46771 - F::cast_from(0.85748036236139473944e-4_f64) * t46776 - F::cast_from(0.85748036236139473944e-4_f64) * t46780 - F::cast_from(0.24098469264142313933e-5_f64) * t46787 - F::cast_from(0.22866142996303859718e-3_f64) * t46789 + F::cast_from(0.71456696863449561619e-5_f64) * t46793 - F::cast_from(0.42874018118069736972e-4_f64) * t46797;
    t48683
}
