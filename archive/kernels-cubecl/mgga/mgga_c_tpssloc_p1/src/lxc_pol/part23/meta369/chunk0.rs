//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1169/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1169<F: Float>(t44620: F, t974: F, t43763: F, t461: F, t1176: F, t2402: F, t42339: F, t466: F, t11715: F, t42341: F, t11721: F, t23508: F) -> (F, F, F, F, F, F) {
    let t44621 = t974 * t44620;
    let t44622 = t461 * t43763;
    let t44633 = t2402 * t1176;
    let t44696 = t466 * t42339;
    let t44698 = t44696 * t42341 * t11715;
    let t44701 = t23508 * t11721;
    (t44621, t44622, t44633, t44696, t44698, t44701)
}
