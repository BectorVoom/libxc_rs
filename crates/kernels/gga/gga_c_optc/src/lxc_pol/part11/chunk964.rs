//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 964/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk964<F: Float>(t26214: F, t406: F, t26261: F, t1135: F, t508: F, t438: F, t935: F, t2849: F, t3107: F, t449: F, t24502: F, t465: F, t3145: F, t8428: F, t3102: F, t26255: F, t8425: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t26745 = t406 * t26214;
    let t26780 = 0.96141975308641975307e-1 * t26261;
    let t26808 = 0.17757530864197530864e0 * t26261;
    let t26836 = 0.18467901234567901234e0 * t26261;
    let t26869 = t508 * t1135;
    let t26881 = t935 * t438;
    let t26888 = t3107 * t2849;
    let t26910 = t508 * t449;
    let t26940 = t465 * t24502;
    let t26989 = t3145 * t8428;
    let t27031 = t3102 * t24502;
    let t27037 = t8425 * t26255;
    (t26745, t26780, t26808, t26836, t26869, t26881, t26888, t26910, t26940, t26989, t27031, t27037)
}
