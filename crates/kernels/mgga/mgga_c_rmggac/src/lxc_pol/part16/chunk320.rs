//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 320/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk320<F: Float>(t1173: F, t671: F, t674: F, t128: F, t495: F, t118: F, t1986: F) -> (F, F, F) {
    let t1993 = t671 * t1173;
    let t1994 = t1993 * t674;
    let t1995 = t128 * t495;
    let t1996 = t118 * t1995;
    let t1997 = t1986 * t1996;
    (t1993, t1994, t1997)
}
