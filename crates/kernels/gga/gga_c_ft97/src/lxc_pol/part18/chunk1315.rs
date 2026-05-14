//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1315/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1315<F: Float>(t105386: F, t105390: F, t105396: F, t105400: F, t105404: F, t105409: F, t105412: F, t105414: F, t105417: F, t105421: F, t96050: F, t96051: F, t1359: F, t9132: F, t12338: F, t1901: F) -> (F, F) {
    let t105423 = t105386 / 6.0 - 2.0 / 3.0 * t105390 - t105396 / 3.0 + 12.0 * t105400 - t105404 / 3.0 + t105409 / 3.0 - t105412 - 11.0 / 9.0 * t105414 + t105417 - t96050 + 3.0 / 4.0 * t105421 + t96051;
    let t105425 = t9132 * t1359;
    let t105427 = t1901 * t105425 * t12338;
    (t105423, t105427)
}
