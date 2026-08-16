//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2598/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2598(t47844: f64, t14099: f64, t2453: f64, t9676: f64, t14109: f64, t9680: f64, t9685: f64, t1424: f64, t4076: f64, t4131: f64, t47466: f64, t47472: f64, t47474: f64, t47478: f64, t47483: f64, t47487: f64, t47832: f64, t47835: f64, t47838: f64, t47839: f64, t5715: f64, t5774: f64, t9652: f64) -> f64 {
    let t47845 = 0.69394917116090352834e-2_f64 * t47844;
    let t47856 = t2453 * t14099;
    let t47857 = t47856 * t9676;
    let t47858 = 0.34697458558045176417e-2_f64 * t47857;
    let t47860 = t9680 * t14109 * t9685;
    let t47862 = 0.11708928647259339623e0_f64 * t47832 - t47835 - 0.34697458558045176417e-2_f64 * t47466 - t47838 + 0.43902994552903410656e-1_f64 * t47839 - 0.32927245914677557992e-1_f64 * t47472 + t47845 + 0.39512695097613069591e1_f64 * t1424 * t4076 * t5774 * t4131 + 0.39512695097613069591e1_f64 * t5715 * t9652 + 0.91069445034239308175e-1_f64 * t47474 - 0.91069445034239308175e-1_f64 * t47478 + 0.69394917116090352834e-2_f64 * t47483 + 0.13878983423218070566e-1_f64 * t47487 - t47858 - 0.7805952431506226415e-1_f64 * t47860;
    t47862
}
