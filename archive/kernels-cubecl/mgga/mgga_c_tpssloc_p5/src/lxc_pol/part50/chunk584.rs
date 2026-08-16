//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 584/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk584<F: Float>(t1625: F, t990: F, t4343: F, t977: F, t2979: F, t4338: F, t1539: F, t248: F, t3051: F, t1041: F, t1616: F, t884: F) -> (F, F, F, F, F, F) {
    let t4559 = t990 * t1625;
    let t4562 = t977 * t4343;
    let t4565 = t2979 * t4338;
    let t4571 = t248 * t3051 * t1539;
    let t4572 = t1041 * t4571;
    let t4574 = t1616 * t884;
    (t4559, t4562, t4565, t4571, t4572, t4574)
}
