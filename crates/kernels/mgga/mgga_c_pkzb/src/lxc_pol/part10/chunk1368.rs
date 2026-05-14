//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1368/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1368<F: Float>(t1878: F, t218: F, t3761: F, t675: F, t9821: F, t2203: F, t27310: F, t1167: F, t219: F, t7945: F, t9828: F, t9832: F, t2185: F, t3730: F, t824: F, t9795: F) -> (F, F, F, F, F, F, F, F) {
    let t27358 = t218 * t1878 * t3761;
    let t27361 = t218 * t675 * t9821;
    let t27363 = t2203 * t27310;
    let t27367 = t218 * t219 * t1167 * t7945;
    let t27370 = t218 * t675 * t9828;
    let t27373 = t218 * t675 * t9832;
    let t27377 = t218 * t219 * t2185 * t3730;
    let t27381 = t218 * t219 * t824 * t9795;
    (t27358, t27361, t27363, t27367, t27370, t27373, t27377, t27381)
}
