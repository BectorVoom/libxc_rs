//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 332/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk332<F: Float>(t1985: F, t1986: F, t27: F, t89: F, t538: F, t132: F, t139: F, t128: F, t131: F) -> (F, F, F, F, F, F) {
    let t1987 = t1985 * t1986;
    let t1989 = t89 * t27 * t1987;
    let t1991 = t538 * t538;
    let t1992 = t1991 * t132;
    let t1993 = t1992 * t139;
    let t1995 = t128 * t131;
    (t1987, t1989, t1991, t1992, t1993, t1995)
}
