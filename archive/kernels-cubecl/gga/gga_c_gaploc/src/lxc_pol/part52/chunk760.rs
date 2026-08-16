//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 760/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk760<F: Float>(t2021: F, t36515: F, t11756: F, t783: F, t321: F, t3614: F, t107: F, t787: F, t11844: F, t1980: F, t11848: F, t35445: F, t739: F) -> (F, F, F, F, F, F, F) {
    let t36516 = t2021 * t36515;
    let t36590 = t11756 * t783;
    let t36610 = t321 * t3614;
    let t36612 = t787 * t36610 * t107;
    let t36632 = t1980 * t11844;
    let t36635 = t1980 * t11848;
    let t36654 = t739 * t35445;
    (t36516, t36590, t36610, t36612, t36632, t36635, t36654)
}
