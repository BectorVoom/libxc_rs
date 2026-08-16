//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 711/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk711<F: Float>(t1567: F, t2890: F, t8442: F, t1001: F, t8422: F, t1901: F) -> (F, F, F) {
    let t8443 = t2890 * t1567;
    let t8444 = t8442 * t8443;
    let t8446 = t8422 * t1001;
    let t8448 = F::cast_from(1.0_f64) / t1901;
    (t8444, t8446, t8448)
}
