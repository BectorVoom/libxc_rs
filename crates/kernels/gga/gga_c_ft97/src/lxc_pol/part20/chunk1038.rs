//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1038/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1038<F: Float>(t1486: F, t25001: F, t681: F, t2399: F, t6308: F, t6310: F, t6323: F, t25005: F, t25027: F, t25029: F, t43548: F, t91: F, t25162: F, t25167: F, t25171: F, t25175: F) -> (F, F, F, F, F, F, F, F, F) {
    let t99452 = t1486 * t681 * t25001;
    let t99457 = t6308 * t2399 * t6310;
    let t99467 = t1486 * t2399 * t6323;
    let t99470 = t1486 * t681 * t25005;
    let t99473 = t25027 * t681 * t25029;
    let t99475 = t91 * t43548;
    let t99492 = t25162 * t25167;
    let t99504 = t25162 * t25171;
    let t99506 = t25162 * t25175;
    (t99452, t99457, t99467, t99470, t99473, t99475, t99492, t99504, t99506)
}
