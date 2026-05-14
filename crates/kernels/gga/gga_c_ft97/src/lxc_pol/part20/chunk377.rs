//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 377/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk377<F: Float>(t2607: F, t3880: F, t2606: F, t2360: F, t258: F, t505: F, t992: F) -> (F, F, F, F) {
    let t3881 = t2607 * t3880;
    let t3882 = t2606 * t3881;
    let t3885 = t258 * t2360;
    let t3886 = t992 * t505;
    (t3881, t3882, t3885, t3886)
}
