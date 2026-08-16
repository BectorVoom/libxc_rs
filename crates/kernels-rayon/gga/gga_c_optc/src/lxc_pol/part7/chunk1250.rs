//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1250/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1250(t2607: f64, t8209: f64, t8215: f64, t2789: f64, t7958: f64, t123: f64, t25776: f64, t25781: f64, t25783: f64, t25788: f64, t25789: f64, t25791: f64, t25793: f64, t25799: f64, t25804: f64, t25806: f64, t2701: f64, t2773: f64, t2797: f64, t323: f64, t8092: f64, t8177: f64, t8208: f64, t8214: f64) -> f64 {
    let t25807 = t8209 * t2607;
    let t25811 = t8215 * t2607;
    let t25815 = t7958 * t2789;
    let t25817 = 0.26372962023724310886e4_f64 * t2773 * t323 * t25776 * t123 - 0.49917948358154037253e1_f64 * t25781 - 0.34034964789650479946e0_f64 * t25783 - t25788 - 0.1133330683113201024e1_f64 * t25789 - 0.34343354033733364364e0_f64 * t25791 - 0.47768371634597164836e-1_f64 * t25793 - 0.3399992049339603072e1_f64 * t8177 * t2701 + 0.31957282085435444036e5_f64 * t25799 - 0.30909018630360027928e0_f64 * t2797 * t8092 + 0.38636273287950034909e-1_f64 * t25804 + 0.20408653907080965924e7_f64 * t8208 * t25806 * t25807 - 0.20408653907080965924e7_f64 * t8214 * t25806 * t25811 + 0.15802725909364645562e4_f64 * t25815;
    t25817
}
