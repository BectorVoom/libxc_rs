//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1029/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1029<F: Float>(t108517: F, t141116: F, t35504: F, t52: F, t709: F, t35367: F, t6050: F, t6815: F, t140943: F, t33434: F, t35431: F, t226: F, t33350: F) -> (F, F, F, F, F, F) {
    let t150640 = t108517 * t141116;
    let t150655 = t52 * t35504 * t709;
    let t150658 = t35367 * t6050;
    let t150659 = t6815 * t150658;
    let t150662 = t33434 * t140943 * t35431;
    let t150664 = t33350 * t226;
    (t150640, t150655, t150658, t150659, t150662, t150664)
}
