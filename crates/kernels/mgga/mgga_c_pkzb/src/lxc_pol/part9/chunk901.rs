//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 901/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk901<F: Float>(t7382: F, t7406: F, t703: F, t1070: F, t1898: F, t1902: F, t1084: F, t5766: F, t1850: F, t2783: F, t1096: F, t1108: F, t1911: F, t1933: F, t1941: F, t1950: F, t1980: F, t2796: F, t2816: F, t2849: F, t5820: F, t5845: F, t5877: F, t695: F, t7309: F, t7314: F, t7315: F, t7324: F) -> (F, F, F, F, F, F, F) {
    let t7407 = t7382 + t7406;
    let t7408 = t7407 * t703;
    let t7411 = t1070 * t1898;
    let t7413 = 0.16081979498692535067e2 * t7411 * t1902;
    let t7415 = 1.0 * t5766 * t1084;
    let t7417 = 2.0 * t1850 * t2783;
    let t7418 = 0.10254018858216406658e4 * t5845 * t7309 - t7314 + 0.17315859105681463759e2 * t7315 * t1980 + 0.5848223622634646207e0 * t5877 * t1108 + 0.11696447245269292414e1 * t1950 * t2849 + 1.0 * t2796 * t1933 + 0.32163958997385070134e2 * t7324 * t1941 + 1.0 * t5820 * t1096 + 2.0 * t1911 * t2816 + 1.0 * t695 * t7408 - t7413 - t7415 - t7417;
    (t7407, t7408, t7411, t7413, t7415, t7417, t7418)
}
