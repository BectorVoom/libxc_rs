//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1235/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1235<F: Float>(t1512: F, t424: F, t23518: F, t487: F, t5239: F, t17643: F, t4305: F, t15066: F, t15067: F, t5096: F, t43636: F, t5101: F, t1094: F, t1102: F, t58311: F, t8749: F) -> (F, F, F, F, F, F, F) {
    let t58547 = 1.0 / t424 / t1512;
    let t58560 = t23518 * t487;
    let t58563 = t5239 * t5239;
    let t58572 = 0.2077890707925103596e3 * t4305 * t17643;
    let t58581 = t15066 * t15067 * t5096;
    let t58585 = t43636 * t15067 * t5101;
    let t58591 = 0.1403573615389248977e2 * t1102 * t8749 * t58311 * t1094;
    (t58547, t58560, t58563, t58572, t58581, t58585, t58591)
}
