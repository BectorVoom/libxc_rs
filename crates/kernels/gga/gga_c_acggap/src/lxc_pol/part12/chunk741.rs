//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 741/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk741<F: Float>(t1314: F, t142: F, t8806: F, t1318: F, t7436: F, t2313: F, t361: F, t2030: F, t1298: F, t599: F, t2317: F, t2060: F, t2031: F, t507: F, t2061: F, t2314: F, t7447: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t8807 = t142 * t1314;
    let t8808 = t8806 * t8807;
    let t8810 = t142 * t1318;
    let t8811 = t7436 * t8810;
    let t8813 = t361 * t2313;
    let t8814 = t2030 * t8813;
    let t8816 = t599 * t1298;
    let t8817 = t142 * t8816;
    let t8818 = t2030 * t8817;
    let t8820 = t361 * t2317;
    let t8821 = t2060 * t8820;
    let t8823 = t507 * t2031;
    let t8824 = t2030 * t8823;
    let t8826 = t507 * t2061;
    let t8827 = t2060 * t8826;
    let t8829 = t7447 * t2314;
    (t8807, t8808, t8810, t8811, t8813, t8814, t8816, t8817, t8818, t8820, t8821, t8823, t8824, t8826, t8827, t8829)
}
