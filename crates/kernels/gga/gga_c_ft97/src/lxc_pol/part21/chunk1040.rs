//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1040/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1040<F: Float>(t1368: F, t1771: F, t5902: F, t1637: F, t5921: F, t89: F, t1369: F, t5905: F, t5890: F, t5892: F, t1636: F, t5925: F, t40280: F, t91: F, t1900: F, t2086: F, t6: F) -> (F, F, F, F, F, F, F, F) {
    let t95099 = t1368 * t1771;
    let t95100 = t95099 * t5902;
    let t95177 = t89 * t1637 * t5921;
    let t95225 = t1369 * t1637 * t5905;
    let t95228 = t5890 * t1637 * t5892;
    let t95242 = t89 * t1636 * t5925;
    let t95262 = t91 * t40280;
    let t95292 = t91 * t2086 * t6 * t1900;
    (t95099, t95100, t95177, t95225, t95228, t95242, t95262, t95292)
}
