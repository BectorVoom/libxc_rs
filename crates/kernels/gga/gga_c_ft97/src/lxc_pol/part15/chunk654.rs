//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 654/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk654<F: Float>(t20395: F, t488: F, t83: F, t3238: F, t4589: F, t4551: F, t979: F, t8418: F, t10969: F, t110: F, t20113: F, t8411: F, t1871: F, t4436: F, t986: F, t16246: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t20396 = t488 * t20395;
    let t20397 = t83 * t20396;
    let t20400 = t3238 * t4589;
    let t20401 = t83 * t20400;
    let t20403 = t4551 * t979;
    let t20404 = t8418 * t20403;
    let t20405 = t83 * t20404;
    let t20408 = t10969 * t4551;
    let t20409 = t83 * t20408;
    let t20413 = t8411 * t110 * t20113;
    let t20417 = t1871 * t986 * t4436;
    let t20420 = t16246 * t979;
    (t20396, t20397, t20400, t20401, t20403, t20404, t20405, t20408, t20409, t20413, t20417, t20420)
}
