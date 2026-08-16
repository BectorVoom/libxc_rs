//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1085/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1085(t2259: f64, t26971: f64, t2257: f64, t7964: f64, t7974: f64, t27482: f64, t2264: f64, t4479: f64, t1628: f64, t7996: f64, t2167: f64, t4527: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t27651 = t26971 * t2259;
    let t27653 = 0.7722800925925925926e-4_f64 * t2257 * t27651;
    let t27654 = t7964 * t7974;
    let t27668 = 0.38691203703703703703e-3_f64 * t27482;
    let t27702 = t2264 * t4479;
    let t27710 = t7996 * t1628;
    let t27733 = t4527 * t2167;
    (t27651, t27653, t27654, t27668, t27702, t27710, t27733)
}
