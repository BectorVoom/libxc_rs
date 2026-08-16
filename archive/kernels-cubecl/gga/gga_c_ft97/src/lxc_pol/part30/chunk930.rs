//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 930/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk930<F: Float>(t7440: F, t771: F, t33567: F, t92: F, t458: F, t7436: F, t6005: F, t24223: F, t7437: F, t1403: F, t33278: F, t681: F) -> (F, F, F, F, F, F) {
    let t140574 = t7440 * t771;
    let t140579 = t33567 * t92;
    let t140582 = t7436 * t458;
    let t140583 = t140582 * t6005;
    let t140585 = t7437 * t24223;
    let t140588 = t1403 * t681 * t33278;
    (t140574, t140579, t140582, t140583, t140585, t140588)
}
