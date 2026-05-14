//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 488/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk488<F: Float>(t332: F, t6399: F, t1511: F, t5: F, t2: F, t4914: F, t4: F, t26: F) -> (F, F, F, F) {
    let t6400 = t6399 * t332;
    let t6403 = t5 * t1511;
    let t6743 = t4914 * t2;
    let t6744 = t6743 * t4;
    let t6745 = t6744 * t26;
    (t6400, t6403, t6744, t6745)
}
