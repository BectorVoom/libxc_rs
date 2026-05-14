//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 663/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk663<F: Float>(t102: F, t133: F, t115: F, t2139: F, t2048: F, t592: F, t1872: F, t544: F, t2204: F, t732: F, t43: F, t97: F, t50: F, t99: F, t1998: F, t509: F) -> (F, F, F, F, F, F, F, F) {
    let t6654 = t133 * t102;
    let t6680 = t2139 * t115;
    let t6695 = t2048 * t592;
    let t6696 = 96.0 * t6695;
    let t6709 = 12.0 * t544 * t1872;
    let t6711 = 35.0 / 3.0 * t732 * t2204;
    let t6713 = 1.0 / t97 / t43;
    let t6724 = 1.0 / t99 / t50;
    let t6739 = t509 * t1998;
    (t6654, t6680, t6696, t6709, t6711, t6713, t6724, t6739)
}
