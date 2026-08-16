//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 955/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk955<F: Float>(t1775: F, t20789: F, t20786: F, t20818: F, t458: F, t20823: F, t2: F, t20655: F, t20806: F, t20796: F, t20810: F, t20827: F) -> (F, F, F, F, F, F, F, F, F) {
    let t78070 = t1775 * t20789;
    let t78073 = t1775 * t20786;
    let t78089 = t458 * t20818;
    let t78091 = t458 * t20823;
    let t78164 = t2 * t20655;
    let t78179 = t1775 * t20806;
    let t78181 = t1775 * t20796;
    let t78183 = t1775 * t20810;
    let t78185 = t1775 * t20827;
    (t78070, t78073, t78089, t78091, t78164, t78179, t78181, t78183, t78185)
}
