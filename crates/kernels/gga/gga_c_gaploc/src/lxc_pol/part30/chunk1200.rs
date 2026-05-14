//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1200/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1200<F: Float>(t204: F, t34239: F, t587: F, t2487: F, t6711: F, t10241: F, t1265: F) -> (F, F, F) {
    let t34242 = 0.18404604457881959845e2 * t587 * t204 * t34239;
    let t34245 = 0.87421871174939309262e2 * t2487 * t6711 * t34239;
    let t34246 = t10241 * t1265;
    (t34242, t34245, t34246)
}
