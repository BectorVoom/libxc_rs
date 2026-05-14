//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 183/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk183<F: Float>(t79: F, t1301: F, t1302: F, t1291: F, t1295: F, t1300: F) -> (F, F) {
    let t80 = 0.1e-59 < t79;
    let t1303 = t1301 * t1302;
    let t1307 = piecewise3(t80, 2.0 * t1295 - 0.22227677429409423704e-2 * t79 * t1291 - 0.19153082513888888889e-1 * t1300 * t1303, 0.0);
    (t1303, t1307)
}
