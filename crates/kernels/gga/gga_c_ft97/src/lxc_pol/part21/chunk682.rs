//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 682/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk682<F: Float>(t16023: F, t16081: F, t16144: F, t16194: F, t16227: F, t16284: F, t16332: F, t16544: F, t103: F, t16533: F, t15625: F, t4893: F, t648: F, t3664: F, t3659: F, t920: F) -> (F, F, F, F, F) {
    let t16547 = t16023 + t16081 + t16144 + t16194 + t16227 + t16284 + t16332 + t16544;
    let t16550 = t16533 * t103;
    let t16579 = -t15625;
    let t16585 = t4893 * t648;
    let t16586 = t16585 * t3664;
    let t16591 = t3659 * t920;
    (t16547, t16550, t16579, t16586, t16591)
}
