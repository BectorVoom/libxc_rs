//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 662/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk662<F: Float>(t16160: F, t8424: F, t1909: F, t16150: F, t3194: F, t18: F, t920: F) -> (F, F, F, F, F) {
    let t16161 = t8424 * t16160;
    let t16162 = t1909 * t16161;
    let t16165 = t3194 * t16150;
    let t16166 = t1909 * t16165;
    let t16169 = t920 * t18;
    (t16161, t16162, t16165, t16166, t16169)
}
