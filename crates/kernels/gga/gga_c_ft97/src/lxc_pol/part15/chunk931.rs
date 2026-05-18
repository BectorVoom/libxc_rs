//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 931/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk931<F: Float>(t5311: F, t8232: F, t5327: F, t5381: F, t2770: F, t5374: F, t5332: F, t38953: F, t5415: F, t2399: F, t5376: F, t89: F) -> (F, F, F, F, F, F, F) {
    let t71532 = t8232 * t5311;
    let t71534 = t8232 * t5327;
    let t71589 = t8232 * t5381;
    let t71624 = t2770 * t5374;
    let t71630 = t8232 * t5332;
    let t71846 = t38953 * t5415;
    let t71907 = t89 * t2399 * t5376;
    (t71532, t71534, t71589, t71624, t71630, t71846, t71907)
}
