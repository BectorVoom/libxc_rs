//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1137/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1137(t16498: f64, t16500: f64, t16502: f64, t16506: f64, t16508: f64, t2609: f64, t5089: f64, t135: f64, t568: f64, t5146: f64, t16532: f64, t1020: f64, t1535: f64, t16526: f64, t16531: f64, t16536: f64, t16539: f64, t1692: f64, t17280: f64, t2718: f64, t2719: f64, t5217: f64, t7191: f64, t7209: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t19693 = 0.17544670867903938621e1_f64 * t16498;
    let t19694 = 0.51947577317044391277e2_f64 * t16500;
    let t19695 = 12.0_f64 * t16502;
    let t19696 = 72.0_f64 * t16506;
    let t19697 = 144.0_f64 * t16508;
    let t19702 = t2609 * t5089;
    let t19703 = 0.10389515463408878255e3_f64 * t19702;
    let t19704 = t135 * t568;
    let t19710 = t2609 * t5146;
    let t19711 = 0.35089341735807877242e1_f64 * t19710;
    let t19712 = 0.48796115851357829289e-1_f64 * t16532;
    let t19719 = 3.0_f64 * t1020 * t1535 * t17280 + 18.0_f64 * t1692 * t2718 * t7191 + 6.0_f64 * t2718 * t2719 * t5217 + 18.0_f64 * t19704 * t7209 + t16526 + t16531 + t16536 - t16539 + t19703 - t19711 + t19712;
    (t19693, t19694, t19695, t19696, t19697, t19703, t19711, t19712, t19719)
}
