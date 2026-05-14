//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 966/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk966<F: Float>(t2855: F, t3107: F, t140: F, t24563: F, t446: F, t464: F, t2849: F, t381: F, t26336: F, t9167: F, t3183: F, t3101: F, t1135: F, t8414: F, t469: F, t8995: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t27152 = t3107 * t2855;
    let t27173 = t446 * t24563 * t140;
    let t27174 = t464 * t27173;
    let t27188 = 1.0 / t381 / t2849;
    let t27189 = t27188 * t26336;
    let t27202 = t9167 * t27173;
    let t27203 = t3107 * t3107;
    let t27209 = t3183 * t27173;
    let t27215 = t3101 * t27173;
    let t27221 = t1135 * t8414;
    let t27276 = 1.0 / t8995 / t469;
    (t27152, t27173, t27174, t27189, t27202, t27203, t27209, t27215, t27221, t27276)
}
