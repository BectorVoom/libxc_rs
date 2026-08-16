//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 507/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk507<F: Float>(t1586: F, t942: F, t432: F, t28: F, t89: F, t1597: F, t383: F, t1594: F, t63: F, t66: F, t77: F, t929: F) -> (F, F, F, F, F, F) {
    let t3013 = t1586 * t942;
    let t3014 = t3013 * t432;
    let t3016 = t89 * t28 * t3014;
    let t3018 = t383 * t1597;
    let t3019 = t1594 * t3018;
    let t3020 = t63 * t66;
    let t3021 = t77 * t929;
    (t3013, t3014, t3016, t3019, t3020, t3021)
}
