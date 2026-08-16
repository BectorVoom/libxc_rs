//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1391/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1391(t1102: f64, t17790: f64, t4224: f64, t5219: f64, t5307: f64, t1512: f64, t5239: f64, t17454: f64, t4305: f64, t15562: f64, t5268: f64, t17502: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t58797 = 0.46785787179641632568e1_f64 * t1102 * t4224 * t17790;
    let t58800 = 0.21053604230838734656e2_f64 * t1102 * t5307 * t5219;
    let t58801 = t5239 * t1512;
    let t58812 = 0.1403573615389248977e2_f64 * t4305 * t17454;
    let t58820 = 0.35089340384731224426e1_f64 * t15562 * t5268;
    let t58822 = 0.23392893589820816284e1_f64 * t4305 * t17502;
    (t58797, t58800, t58801, t58812, t58820, t58822)
}
