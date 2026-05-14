//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 554/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk554<F: Float>(t2603: F, t8392: F, t241: F, t9568: F, t1882: F, t2528: F, t760: F) -> (F, F, F, F, F) {
    let t10012 = t8392 * t2603;
    let t10024 = t9568 * t241;
    let t10048 = t1882 * t2528;
    let t10050 = t760 * t760;
    let t10051 = 1.0 / t10050;
    (t10012, t10024, t10048, t10050, t10051)
}
