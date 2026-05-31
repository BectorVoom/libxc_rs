//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 392/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk392<F: Float>(t165: F, t5843: F, t28: F, t1360: F, t614: F, t1366: F, t1882: F, t1359: F, t160: F) -> (F, F, F, F, F, F) {
    let t5844 = t5843 * t165;
    let t5845 = t28 * t5844;
    let t5848 = t1360 * t614;
    let t5849 = t28 * t5848;
    let t5854 = t1882 * t1366 / F::cast_from(9.0_f64);
    let t5855 = t160 * t1359;
    (t5844, t5845, t5848, t5849, t5854, t5855)
}
