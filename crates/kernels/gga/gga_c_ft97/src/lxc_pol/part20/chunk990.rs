//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 990/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk990<F: Float>(t13927: F, t737: F, t1160: F, t676: F, t2372: F, t2568: F, t1175: F, t9895: F, t17695: F, t61128: F, t17688: F, t2252: F, t342: F, t4910: F, t17708: F, t630: F) -> (F, F, F, F, F, F, F, F) {
    let t67796 = t737 * t13927;
    let t67847 = t676 * t1160;
    let t67996 = t2372 * t2568;
    let t68135 = t9895 * t1175;
    let t69066 = t61128 * t17695 / 9.0;
    let t69068 = 2.0 / 27.0 * t61128 * t17688;
    let t69073 = t342 * t2252 * t4910;
    let t69081 = t342 * t630 * t17708 / 6.0;
    (t67796, t67847, t67996, t68135, t69066, t69068, t69073, t69081)
}
