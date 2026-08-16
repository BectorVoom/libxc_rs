//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 991/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk991<F: Float>(t19345: F, t296: F, t4969: F, t835: F, t882: F, t1255: F, t2862: F, t4162: F, t4167: F, t4246: F, t840: F, t5299: F, t824: F) -> (F, F, F, F, F) {
    let t19346 = t296 * t19345;
    let t19351 = t835 * t882 * t4969;
    let t19355 = t2862 * t1255 * t4162;
    let t19359 = t840 * t4246 * t4167;
    let t19362 = t5299 * t824;
    (t19346, t19351, t19355, t19359, t19362)
}
