//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 654/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk654<F: Float>(t1771: F, t588: F, t1775: F, t2103: F, t2106: F, t2: F, t9114: F, t583: F, t8282: F, t2109: F, t2098: F, t2114: F, t458: F) -> (F, F, F, F, F, F, F, F) {
    let t9179 = t1771 * t588;
    let t9188 = t1775 * t2103;
    let t9190 = t1775 * t2106;
    let t9192 = t9114 * t2;
    let t9202 = t8282 * t583;
    let t9205 = t1775 * t2109;
    let t9207 = t1775 * t2098;
    let t9209 = t458 * t2114;
    (t9179, t9188, t9190, t9192, t9202, t9205, t9207, t9209)
}
