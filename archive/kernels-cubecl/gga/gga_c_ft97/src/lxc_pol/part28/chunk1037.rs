//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1037/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1037<F: Float>(t1317: F, t144991: F, t28: F, t8270: F, t144809: F, t446: F, t7824: F, t144792: F, t144857: F, t38268: F, t34389: F, t376: F, t5665: F) -> (F, F, F, F, F) {
    let t145048 = t1317 * t28 * t8270 * t144991;
    let t145051 = t446 * t7824 * t144809;
    let t145055 = t446 * t7824 * t144792;
    let t145058 = t446 * t38268 * t144857;
    let t145061 = t5665 * t376 * t34389;
    (t145048, t145051, t145055, t145058, t145061)
}
