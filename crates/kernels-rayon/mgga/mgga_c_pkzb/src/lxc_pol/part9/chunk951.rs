//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 951/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk951(t7382: f64, t7406: f64, t703: f64, t1070: f64, t1898: f64, t1902: f64, t1084: f64, t5766: f64, t1850: f64, t2783: f64, t1096: f64, t1108: f64, t1911: f64, t1933: f64, t1941: f64, t1950: f64, t1980: f64, t2796: f64, t2816: f64, t2849: f64, t5820: f64, t5845: f64, t5877: f64, t695: f64, t7309: f64, t7314: f64, t7315: f64, t7324: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7407 = t7382 + t7406;
    let t7408 = t7407 * t703;
    let t7411 = t1070 * t1898;
    let t7413 = 0.16081979498692535067e2_f64 * t7411 * t1902;
    let t7415 = 1.0_f64 * t5766 * t1084;
    let t7417 = 2.0_f64 * t1850 * t2783;
    let t7418 = 0.10254018858216406658e4_f64 * t5845 * t7309 - t7314 + 0.17315859105681463759e2_f64 * t7315 * t1980 + 0.5848223622634646207e0_f64 * t5877 * t1108 + 0.11696447245269292414e1_f64 * t1950 * t2849 + 1.0_f64 * t2796 * t1933 + 0.32163958997385070134e2_f64 * t7324 * t1941 + 1.0_f64 * t5820 * t1096 + 2.0_f64 * t1911 * t2816 + 1.0_f64 * t695 * t7408 - t7413 - t7415 - t7417;
    (t7407, t7408, t7411, t7413, t7415, t7417, t7418)
}
