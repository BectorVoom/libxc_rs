//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 956/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk956<F: Float>(t23265: F, t4611: F, t11854: F, t1901: F, t23183: F, t23311: F, t26428: F, t26451: F, t29958: F, t29963: F, t29967: F, t29971: F, t29975: F, t29979: F, t29982: F, t29988: F, t29992: F, t29996: F, t30001: F, t30005: F, t30009: F, t446: F) -> (F, F, F) {
    let t30012 = t23265 * t4611;
    let t30013 = t11854 * t30012;
    let t30016 = -t23183 + t446 * t29958 / 3.0 + 2.0 / 3.0 * t446 * t29963 + 2.0 / 3.0 * t446 * t29967 + 2.0 / 3.0 * t446 * t29971 + 4.0 / 3.0 * t446 * t29975 + 4.0 / 3.0 * t446 * t29979 + 4.0 / 3.0 * t446 * t29982 + 2.0 / 27.0 * t26428 - 2.0 / 3.0 * t446 * t29988 - 2.0 * t446 * t29992 - 2.0 / 9.0 * t1901 * t29996 - 2.0 / 9.0 * t26451 - t23311 - t446 * t30001 / 3.0 - 2.0 / 3.0 * t446 * t30005 - 2.0 / 9.0 * t1901 * t30009 - 4.0 / 9.0 * t1901 * t30013;
    (t30012, t30013, t30016)
}
