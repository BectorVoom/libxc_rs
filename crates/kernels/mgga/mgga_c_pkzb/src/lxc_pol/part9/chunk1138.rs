//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1138/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1138<F: Float>(t16550: F, t16552: F, t16554: F, t16565: F, t16571: F, t16580: F, t16582: F, t16584: F, t16586: F, t16593: F, t16595: F, t16575: F, t16578: F, t16592: F, t16599: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t19722 = F::new(3.0) * t16550;
    let t19726 = F::cast_from(0.35089341735807877242e1_f64) * t16552;
    let t19729 = F::cast_from(0.17544670867903938621e1_f64) * t16554;
    let t19730 = F::cast_from(0.5848223622634646207e0_f64) * t16565;
    let t19732 = F::new(360.0) * t16571;
    let t19733 = F::cast_from(0.32530743900905219526e-1_f64) * t16580;
    let t19734 = F::cast_from(0.14447919941302971323e1_f64) * t16582;
    let t19735 = F::new(36.0) * t16584;
    let t19736 = F::new(60.0) * t16586;
    let t19737 = F::cast_from(0.10526802520742363173e2_f64) * t16593;
    let t19738 = F::cast_from(0.65061487801810439052e-1_f64) * t16595;
    let t19739 = -t19732 + t16575 + t16578 + t19733 + t19734 - t19735 + t19736 - t16592 - t19737 - t19738 + t16599;
    (t19722, t19726, t19729, t19730, t19732, t19733, t19734, t19735, t19736, t19737, t19738, t19739)
}
