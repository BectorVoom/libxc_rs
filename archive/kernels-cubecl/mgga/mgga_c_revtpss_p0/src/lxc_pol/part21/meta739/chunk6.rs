//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2598/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2598<F: Float>(t47844: F, t14099: F, t2453: F, t9676: F, t14109: F, t9680: F, t9685: F, t1424: F, t4076: F, t4131: F, t47466: F, t47472: F, t47474: F, t47478: F, t47483: F, t47487: F, t47832: F, t47835: F, t47838: F, t47839: F, t5715: F, t5774: F, t9652: F) -> F {
    let t47845 = F::cast_from(0.69394917116090352834e-2_f64) * t47844;
    let t47856 = t2453 * t14099;
    let t47857 = t47856 * t9676;
    let t47858 = F::cast_from(0.34697458558045176417e-2_f64) * t47857;
    let t47860 = t9680 * t14109 * t9685;
    let t47862 = F::cast_from(0.11708928647259339623e0_f64) * t47832 - t47835 - F::cast_from(0.34697458558045176417e-2_f64) * t47466 - t47838 + F::cast_from(0.43902994552903410656e-1_f64) * t47839 - F::cast_from(0.32927245914677557992e-1_f64) * t47472 + t47845 + F::cast_from(0.39512695097613069591e1_f64) * t1424 * t4076 * t5774 * t4131 + F::cast_from(0.39512695097613069591e1_f64) * t5715 * t9652 + F::cast_from(0.91069445034239308175e-1_f64) * t47474 - F::cast_from(0.91069445034239308175e-1_f64) * t47478 + F::cast_from(0.69394917116090352834e-2_f64) * t47483 + F::cast_from(0.13878983423218070566e-1_f64) * t47487 - t47858 - F::cast_from(0.7805952431506226415e-1_f64) * t47860;
    t47862
}
