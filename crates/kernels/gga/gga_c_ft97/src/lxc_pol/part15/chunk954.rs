//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 954/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk954<F: Float>(t1882: F, t20546: F, t1546: F, t20667: F, t89: F, t20549: F, t7780: F, t1984: F, t20655: F, t20664: F, t376: F, t1775: F, t20793: F) -> (F, F, F, F, F, F) {
    let t78001 = t1882 * t20546;
    let t78012 = t89 * t1546 * t20667;
    let t78015 = t89 * t7780 * t20549;
    let t78017 = t1984 * t20655;
    let t78027 = t89 * t376 * t20664;
    let t78068 = t1775 * t20793;
    (t78001, t78012, t78015, t78017, t78027, t78068)
}
