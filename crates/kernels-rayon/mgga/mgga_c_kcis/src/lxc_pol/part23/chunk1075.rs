//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1075/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1075(t4463: f64, t7984: f64, t6176: f64, t2259: f64, t26971: f64, t2257: f64, t7964: f64, t7974: f64, t3801: f64, t7979: f64, t1600: f64, t27482: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t27647 = t7984 * t4463;
    let t27648 = t6176 * t27647;
    let t27651 = t26971 * t2259;
    let t27653 = 0.7722800925925925926e-4_f64 * t2257 * t27651;
    let t27654 = t7964 * t7974;
    let t27664 = t7979 * t3801;
    let t27665 = t1600 * t27664;
    let t27668 = 0.38691203703703703703e-3_f64 * t27482;
    (t27647, t27648, t27651, t27653, t27654, t27664, t27665, t27668)
}
