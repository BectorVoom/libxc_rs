//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1244/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1244<F: Float>(t1131: F, t24437: F, t2574: F, t3938: F, t6119: F, t1154: F, t3821: F, t27819: F, t5092: F, t713: F, t24546: F, t4965: F, t6118: F, t9744: F, t30972: F, t96925: F) -> (F, F, F, F, F) {
    let t123909 = t24437 * t2574 * t6119 * t3938 * t1131;
    let t123914 = t24437 * t2574 * t6119 * t1154 * t3821;
    let t123919 = t27819 * t2574 * t6119 * t5092 * t713;
    let t123923 = t6118 * t9744 * t24546 * t4965;
    let t123925 = t96925 * t30972;
    (t123909, t123914, t123919, t123923, t123925)
}
