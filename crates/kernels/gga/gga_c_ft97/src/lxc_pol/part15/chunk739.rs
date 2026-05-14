//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 739/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk739<F: Float>(t10261: F, t21978: F, t27: F, t89: F, t4056: F, t5299: F, t193: F, t20489: F, t792: F, t666: F, t21181: F, t2660: F, t21204: F, t835: F, t446: F, t14738: F, t5284: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t21979 = t10261 * t21978;
    let t21981 = t89 * t27 * t21979;
    let t21982 = t4056 * t5299;
    let t21984 = t89 * t193 * t21982;
    let t21985 = t792 * t20489;
    let t21987 = t89 * t666 * t21985;
    let t21989 = t2660 * t21181;
    let t21991 = t89 * t666 * t21989;
    let t21993 = t835 * t21204;
    let t21994 = t446 * t21993;
    let t21996 = t14738 * t5284;
    (t21979, t21981, t21982, t21984, t21985, t21987, t21989, t21991, t21993, t21994, t21996)
}
