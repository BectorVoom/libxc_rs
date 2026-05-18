//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 339/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk339<F: Float>(t1882: F, t449: F, t104: F, t1637: F, t89: F, t454: F, t494: F, t27: F, t444: F, t443: F) -> (F, F, F, F, F) {
    let t1883 = t1882 * t449;
    let t1887 = F::new(4.0) / F::new(27.0) * t89 * t1637 * t104;
    let t1888 = t1882 * t454;
    let t1890 = t1882 * t494;
    let t1900 = t444 * t27;
    let t1901 = t443 * t1900;
    (t1883, t1887, t1888, t1890, t1901)
}
