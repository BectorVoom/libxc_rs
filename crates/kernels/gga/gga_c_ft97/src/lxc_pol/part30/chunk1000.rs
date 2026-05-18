//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1000/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1000<F: Float>(t140744: F, t150056: F, t33292: F, t140763: F, t150060: F, t150064: F, t24437: F, t2574: F, t27845: F, t6119: F, t10157: F, t27878: F, t6118: F) -> (F, F, F, F, F) {
    let t150096 = t33292 * t140744 * t150056;
    let t150099 = t33292 * t140763 * t150060;
    let t150102 = t33292 * t140763 * t150064;
    let t150106 = t24437 * t2574 * t6119 * t27845;
    let t150109 = t6118 * t10157 * t6119 * t27878;
    (t150096, t150099, t150102, t150106, t150109)
}
