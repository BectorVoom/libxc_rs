//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1253/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1253<F: Float>(t1107: F, t3126: F, t4457: F, t4459: F, t4464: F, t4465: F, t26936: F, t4435: F, t4437: F, t1124: F, t3103: F, t1162: F, t2367: F, t8483: F, t1150: F, t9025: F) -> (F, F, F, F, F, F) {
    let t27719 = t1107 * t3126;
    let t27721 = t4457 * t27719 * t4459;
    let t27724 = t4464 * t27719 * t4465;
    let t27730 = t4435 * t26936 * t4437;
    let t27733 = t3103 * t26936 * t1124;
    let t27736 = t1162 * t2367 * t8483;
    let t27744 = t1150 * t2367 * t9025;
    (t27721, t27724, t27730, t27733, t27736, t27744)
}
