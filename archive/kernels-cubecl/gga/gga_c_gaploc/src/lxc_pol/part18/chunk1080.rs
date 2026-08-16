//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1080/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1080<F: Float>(t3394: F, t486: F, t4144: F, t987: F, t4245: F, t4398: F, t8410: F, t1: F, t25760: F, t1415: F, t1519: F, t2876: F) -> (F, F, F, F, F, F, F) {
    let t25893 = t3394 * t486;
    let t25955 = t987 * t4144;
    let t26011 = t987 * t4245;
    let t26122 = t4398 * t8410;
    let t26126 = t25760 * t1;
    let t26127 = t1415 * t26126;
    let t26244 = t2876 * t1519;
    (t25893, t25955, t26011, t26122, t26126, t26127, t26244)
}
