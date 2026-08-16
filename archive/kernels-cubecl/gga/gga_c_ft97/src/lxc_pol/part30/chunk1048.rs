//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1048/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1048<F: Float>(t150056: F, t41825: F, t446: F, t150060: F, t9770: F, t1882: F, t35522: F, t1434: F, t35517: F, t681: F, t193: F, t24191: F, t6837: F, t89: F) -> (F, F, F, F, F) {
    let t150980 = t446 * t41825 * t150056;
    let t150983 = t446 * t9770 * t150060;
    let t150985 = t1882 * t35522;
    let t150988 = t1434 * t681 * t35517;
    let t150992 = t89 * t193 * t24191 * t6837;
    (t150980, t150983, t150985, t150988, t150992)
}
