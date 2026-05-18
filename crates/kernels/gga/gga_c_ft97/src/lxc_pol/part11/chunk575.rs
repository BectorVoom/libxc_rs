//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 575/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk575<F: Float>(t1613: F, t77: F, t373: F, t1608: F, t384: F, t39: F, t1689: F, t1691: F, t1696: F, t1609: F, t1593: F, t1632: F, t6: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7998 = t77 * t1613;
    let t7999 = t7998 * t373;
    let t8000 = t1608 * t7999;
    let t8001 = t384 * t39;
    let t8002 = t1689 * t1691;
    let t8003 = t8002 * t1696;
    let t8007 = t77 * t1609;
    let t8008 = t8007 * t1593;
    let t8009 = t1608 * t8008;
    let t8010 = t1632 * t6;
    (t7998, t7999, t8000, t8001, t8002, t8003, t8007, t8008, t8009, t8010)
}
