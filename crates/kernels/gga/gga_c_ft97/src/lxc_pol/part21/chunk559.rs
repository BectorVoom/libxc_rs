//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 559/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk559<F: Float>(t1570: F, t487: F, t7775: F, t8192: F, t8189: F, t1851: F, t480: F) -> (F, F, F, F, F) {
    let t8424 = t487 * t1570;
    let t8443 = 4.0 / 27.0 * t7775;
    let t8451 = 4.0 / 9.0 * t8192;
    let t8455 = 28.0 / 81.0 * t8189;
    let t8466 = t480 * t1851;
    (t8424, t8443, t8451, t8455, t8466)
}
