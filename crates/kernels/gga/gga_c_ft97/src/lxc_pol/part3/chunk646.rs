//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 646/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk646<F: Float>(t1160: F, t2492: F, t265: F, t9895: F, t2568: F, t737: F, t762: F, t2486: F, t9802: F, t1882: F, t3983: F, t3839: F, t1140: F, t8232: F, t3848: F, t1170: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t14159 = t2492 * t1160;
    let t14163 = t9895 * t265;
    let t14175 = t737 * t2568;
    let t14182 = t737 * t762;
    let t14187 = t2486 * t762;
    let t14196 = t2492 * t265;
    let t14200 = t9802 * t265;
    let t14212 = 2.0 / 9.0 * t1882 * t3983;
    let t14223 = 4.0 / 9.0 * t1882 * t3839;
    let t14224 = t8232 * t1140;
    let t14232 = 2.0 / 27.0 * t1882 * t3848;
    let t14233 = t8232 * t1170;
    (t14159, t14163, t14175, t14182, t14187, t14196, t14200, t14212, t14223, t14224, t14232, t14233)
}
