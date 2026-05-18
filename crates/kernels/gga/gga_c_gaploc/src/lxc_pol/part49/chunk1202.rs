//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1202/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1202<F: Float>(t13805: F, t1580: F, t1445: F, t47026: F, t597: F, t46945: F, t40449: F, t40452: F, t40455: F, t40458: F, t13810: F, t4950: F) -> (F, F, F, F, F, F, F, F) {
    let t48131 = t1580 * t13805;
    let t48134 = t597 * t1445 * t47026;
    let t48137 = t597 * t1445 * t46945;
    let t48140 = F::new(0.63904876589867916128e-1) * t40449;
    let t48141 = F::new(0.31952438294933958064e0) * t40452;
    let t48142 = F::new(0.51123901271894332903e0) * t40455;
    let t48143 = F::new(0.38342925953920749677e0) * t40458;
    let t48144 = t4950 * t13810;
    (t48131, t48134, t48137, t48140, t48141, t48142, t48143, t48144)
}
