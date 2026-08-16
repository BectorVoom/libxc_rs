//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1050/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1050<F: Float>(t193: F, t27742: F, t6008: F, t89: F, t35529: F, t681: F, t33253: F, t3821: F, t150202: F, t24438: F, t6118: F, t150206: F, t27762: F) -> (F, F, F, F, F) {
    let t151001 = t89 * t193 * t6008 * t27742;
    let t151004 = t89 * t681 * t35529;
    let t151008 = t89 * t193 * t33253 * t3821;
    let t151011 = t6118 * t24438 * t150202;
    let t151014 = t6118 * t27762 * t150206;
    (t151001, t151004, t151008, t151011, t151014)
}
