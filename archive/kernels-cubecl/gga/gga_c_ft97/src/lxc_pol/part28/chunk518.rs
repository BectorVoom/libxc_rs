//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 518/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk518<F: Float>(t383: F, t7857: F, t1598: F, t66: F, t1630: F, t929: F, t25: F, t78: F, t1593: F, t2248: F, t422: F, t110: F, t1786: F) -> (F, F, F, F, F, F, F, F) {
    let t11119 = t7857 * t383;
    let t11120 = t1598 * t66;
    let t11121 = t11119 * t11120;
    let t11233 = t1630 * t929;
    let t11240 = t78 * t25;
    let t11247 = t1593 * t929;
    let t11280 = t2248 * t422;
    let t11468 = t1786 * t110;
    (t11119, t11120, t11121, t11233, t11240, t11247, t11280, t11468)
}
