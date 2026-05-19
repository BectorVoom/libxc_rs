//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1137/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1137<F: Float>(t16498: F, t16500: F, t16502: F, t16506: F, t16508: F, t2609: F, t5089: F, t135: F, t568: F, t5146: F, t16532: F, t1020: F, t1535: F, t16526: F, t16531: F, t16536: F, t16539: F, t1692: F, t17280: F, t2718: F, t2719: F, t5217: F, t7191: F, t7209: F) -> (F, F, F, F, F, F, F, F, F) {
    let t19693 = F::cast_from(0.17544670867903938621e1_f64) * t16498;
    let t19694 = F::cast_from(0.51947577317044391277e2_f64) * t16500;
    let t19695 = F::new(12.0) * t16502;
    let t19696 = F::new(72.0) * t16506;
    let t19697 = F::new(144.0) * t16508;
    let t19702 = t2609 * t5089;
    let t19703 = F::cast_from(0.10389515463408878255e3_f64) * t19702;
    let t19704 = t135 * t568;
    let t19710 = t2609 * t5146;
    let t19711 = F::cast_from(0.35089341735807877242e1_f64) * t19710;
    let t19712 = F::cast_from(0.48796115851357829289e-1_f64) * t16532;
    let t19719 = F::new(3.0) * t1020 * t1535 * t17280 + F::new(18.0) * t1692 * t2718 * t7191 + F::new(6.0) * t2718 * t2719 * t5217 + F::new(18.0) * t19704 * t7209 + t16526 + t16531 + t16536 - t16539 + t19703 - t19711 + t19712;
    (t19693, t19694, t19695, t19696, t19697, t19703, t19711, t19712, t19719)
}
