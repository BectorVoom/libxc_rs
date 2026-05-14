//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 951/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk951<F: Float>(t304: F, t330: F, t5557: F, t679: F, t8: F, t123: F, t2084: F, t160: F, t23: F, t268: F, t1933: F, t62: F, t1375: F, t1381: F, t4348: F, t498: F) -> (F, F, F, F, F, F, F) {
    let t16710 = t304 / t5557 / t330;
    let t16788 = t8 * t679;
    let t16879 = t2084 * t123;
    let t16880 = t16879 * t160;
    let t16889 = t23 * t268;
    let t16922 = t62 * t1933;
    let t17277 = t1375 * t1381;
    let t17288 = t498 * t4348;
    (t16710, t16788, t16880, t16889, t16922, t17277, t17288)
}
