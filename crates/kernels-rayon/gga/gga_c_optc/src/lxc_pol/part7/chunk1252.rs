//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1252/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1252(t25797: f64, t2679: f64, t8114: f64, t8113: f64, t8125: f64, t883: f64, t11398: f64, t2704: f64, t7848: f64, t24567: f64, t935: f64, t19: f64, t23825: f64, t24574: f64, t24575: f64, t25184: f64, t25806: f64, t25821: f64, t25826: f64, t25837: f64, t25843: f64, t2672: f64, t2766: f64, t313: f64, t323: f64, t7427: f64, t7924: f64, t7958: f64, t7996: f64, t7999: f64, t8045: f64, t8130: f64, t8135: f64, t8140: f64, t8194: f64, t914: f64, t930: f64) -> (f64, f64) {
    let t25846 = t8114 * t25797 * t2679;
    let t25849 = t8125 * t883 * t8113;
    let t25852 = t11398 * t8113;
    let t25855 = t2704 * t7848;
    let t25865 = t24567 * t935;
    let t25870 = 0.11852044432023484171e4_f64 * t7958 * t2766 - 0.15146801702008125515e1_f64 * t25821 - 0.69545291918310062836e0_f64 * t930 * t914 * t25184 + 0.34014423178468276542e6_f64 * t8194 * t25806 * t25826 + 0.33037286659193699704e3_f64 * t7427 * t7996 - 0.27022098409157095356e7_f64 * t25837 * t323 * t24575 * t19 - 0.23967961564076583027e5_f64 * t25843 + 0.26631068404529536697e4_f64 * t25846 - 0.25565825668348355228e6_f64 * t25849 * t8130 + 0.19174369251261266421e6_f64 * t25852 * t8135 - 0.35826278725947873626e0_f64 * t25855 + 0.54090782603130048873e0_f64 * t930 * t914 * t7924 * t23825 - 0.24951672488470492992e3_f64 * t8140 * t7999 - 0.24951672488470492992e3_f64 * t8140 * t8045 + 0.81145531355560548285e7_f64 * t24574 * t313 * t25865 * t2672;
    (t25865, t25870)
}
