//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 646/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk646<F: Float>(t26: F, t9555: F, t1771: F, t685: F, t2406: F, t458: F, t2410: F, t2414: F, t322: F, t668: F) -> (F, F, F, F, F, F, F) {
    let t9556 = t26 * t9555;
    let t9557 = 28.0 / 27.0 * t9556;
    let t9558 = t1771 * t685;
    let t9560 = t458 * t2406;
    let t9562 = t458 * t2410;
    let t9564 = t458 * t2414;
    let t9567 = 1.0 / t322 / t668;
    (t9556, t9557, t9558, t9560, t9562, t9564, t9567)
}
