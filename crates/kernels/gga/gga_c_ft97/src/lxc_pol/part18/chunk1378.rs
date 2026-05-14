//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1378/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1378<F: Float>(t1882: F, t26902: F, t6653: F, t8232: F, t1378: F, t1985: F, t105717: F, t105952: F, t12970: F, t13065: F, t13140: F, t144: F, t1647: F, t167: F, t1901: F, t2075: F, t2185: F, t2210: F, t27020: F, t27228: F, t27335: F, t3408: F, t40945: F, t446: F, t574: F, t5975: F, t6725: F, t95820: F, t95827: F, t95829: F, t95849: F, t95859: F) -> (F,) {
    let t107068 = 4.0 / 9.0 * t1882 * t26902;
    let t107077 = t8232 * t6653;
    let t107082 = t1985 * t1378;
    let t107105 = -t107068 + 2.0 * t1901 * t13140 * t27335 * t13065 - 2.0 / 9.0 * t1901 * t2210 * t27020 * t1647 - 4.0 / 27.0 * t107077 - 2.0 / 9.0 * t1901 * t40945 * t27228 - 4.0 / 3.0 * t1901 * t107082 * t12970 + t95820 / 9.0 - 2.0 / 27.0 * t95827 - 2.0 / 27.0 * t95829 + 2.0 / 3.0 * t446 * t2185 * t167 * t105717 - t446 * t574 * t6725 * t2075 / 3.0 - 2.0 / 3.0 * t446 * t574 * t5975 * t3408 - 2.0 / 9.0 * t95849 + t95859 - t446 * t144 * t105952 / 3.0;
    (t107105,)
}
