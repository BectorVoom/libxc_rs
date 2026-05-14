//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 683/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk683<F: Float>(t1397: F, t2371: F, t1: F, t6540: F, t544: F, t1402: F, t2339: F, t447: F, t6509: F, t204: F, t1433: F, t2486: F) -> (F, F, F, F, F, F, F) {
    let t6696 = t1397 * t2371;
    let t6699 = t6540 * t1;
    let t6700 = t544 * t6699;
    let t6703 = t1402 * t2339;
    let t6706 = t6509 * t447;
    let t6707 = t204 * t6706;
    let t6710 = t1433 * t2486;
    (t6696, t6699, t6700, t6703, t6706, t6707, t6710)
}
