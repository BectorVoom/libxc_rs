//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 544/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk544<F: Float>(t1620: F, t5544: F, t39: F, t409: F, t64: F, t25: F, t1602: F, t35: F, t401: F, t1711: F, t6: F) -> (F, F, F, F, F, F, F) {
    let t7861 = t5544 * t1620;
    let t7866 = t409 * t39;
    let t7867 = t64 * t7866;
    let t7876 = t409 * t25;
    let t7877 = t1602 * t7876;
    let t7878 = t35 * t401;
    let t7888 = t1711 * t6;
    let t7889 = t64 * t7888;
    (t7861, t7867, t7876, t7877, t7878, t7888, t7889)
}
