//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 766/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk766<F: Float>(t1882: F, t5395: F, t1248: F, t18: F, t2882: F, t2881: F, t4917: F, t824: F) -> (F, F, F, F, F) {
    let t19453 = t1882 * t5395;
    let t19455 = t18 * t1248;
    let t19456 = t2882 * t19455;
    let t19457 = t2881 * t19456;
    let t19460 = t4917 * t824;
    (t19453, t19455, t19456, t19457, t19460)
}
