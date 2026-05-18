//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1252/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1252<F: Float>(t25797: F, t2679: F, t8114: F, t8113: F, t8125: F, t883: F, t11398: F, t2704: F, t7848: F, t24567: F, t935: F, t19: F, t23825: F, t24574: F, t24575: F, t25184: F, t25806: F, t25821: F, t25826: F, t25837: F, t25843: F, t2672: F, t2766: F, t313: F, t323: F, t7427: F, t7924: F, t7958: F, t7996: F, t7999: F, t8045: F, t8130: F, t8135: F, t8140: F, t8194: F, t914: F, t930: F) -> (F, F) {
    let t25846 = t8114 * t25797 * t2679;
    let t25849 = t8125 * t883 * t8113;
    let t25852 = t11398 * t8113;
    let t25855 = t2704 * t7848;
    let t25865 = t24567 * t935;
    let t25870 = F::new(0.11852044432023484171e4) * t7958 * t2766 - F::new(0.15146801702008125515e1) * t25821 - F::new(0.69545291918310062836e0) * t930 * t914 * t25184 + F::new(0.34014423178468276542e6) * t8194 * t25806 * t25826 + F::new(0.33037286659193699704e3) * t7427 * t7996 - F::new(0.27022098409157095356e7) * t25837 * t323 * t24575 * t19 - F::new(0.23967961564076583027e5) * t25843 + F::new(0.26631068404529536697e4) * t25846 - F::new(0.25565825668348355228e6) * t25849 * t8130 + F::new(0.19174369251261266421e6) * t25852 * t8135 - F::new(0.35826278725947873626e0) * t25855 + F::new(0.54090782603130048873e0) * t930 * t914 * t7924 * t23825 - F::new(0.24951672488470492992e3) * t8140 * t7999 - F::new(0.24951672488470492992e3) * t8140 * t8045 + F::new(0.81145531355560548285e7) * t24574 * t313 * t25865 * t2672;
    (t25865, t25870)
}
