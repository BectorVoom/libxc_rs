//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1292/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1292<F: Float>(t299: F, t115067: F, t115093: F, t107750: F, t107751: F, t111570: F, t13: F, t24868: F, t25520: F) -> (F,) {
    let t300 = 10000000.0 <= t299;
    let t115095 = piecewise3(t300, 0.0, t115067 + t115093);
    let tv4rho3sigma5 = t24868 + t25520 + t107750 + t107751 + t13 * (t111570 + t115095);
    (tv4rho3sigma5,)
}
