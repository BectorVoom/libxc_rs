//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1138/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1138(t16550: f64, t16552: f64, t16554: f64, t16565: f64, t16571: f64, t16580: f64, t16582: f64, t16584: f64, t16586: f64, t16593: f64, t16595: f64, t16575: f64, t16578: f64, t16592: f64, t16599: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t19722 = 3.0_f64 * t16550;
    let t19726 = 0.35089341735807877242e1_f64 * t16552;
    let t19729 = 0.17544670867903938621e1_f64 * t16554;
    let t19730 = 0.5848223622634646207e0_f64 * t16565;
    let t19732 = 360.0_f64 * t16571;
    let t19733 = 0.32530743900905219526e-1_f64 * t16580;
    let t19734 = 0.14447919941302971323e1_f64 * t16582;
    let t19735 = 36.0_f64 * t16584;
    let t19736 = 60.0_f64 * t16586;
    let t19737 = 0.10526802520742363173e2_f64 * t16593;
    let t19738 = 0.65061487801810439052e-1_f64 * t16595;
    let t19739 = -t19732 + t16575 + t16578 + t19733 + t19734 - t19735 + t19736 - t16592 - t19737 - t19738 + t16599;
    (t19722, t19726, t19729, t19730, t19732, t19733, t19734, t19735, t19736, t19737, t19738, t19739)
}
