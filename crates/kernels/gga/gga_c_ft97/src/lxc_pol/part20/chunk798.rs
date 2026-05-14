//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 798/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk798<F: Float>(t24890: F, t2877: F, t2409: F, t6360: F, t2881: F, t1476: F, t870: F) -> (F, F, F, F) {
    let t24891 = t24890 * t2877;
    let t24894 = t6360 * t2409;
    let t24895 = t2881 * t24894;
    let t24898 = t870 * t1476;
    (t24891, t24894, t24895, t24898)
}
