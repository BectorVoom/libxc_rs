//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 180/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk180<F: Float>(t336: F, t337: F, t495: F, t346: F, t345: F, t344: F, t359: F) -> (F, F, F, F) {
    let t500 = t336 * t337 * t495;
    let t503 = t346 * t495;
    let t504 = t345 * t503;
    let t506 = -t344 - t504 / F::cast_from(4.0_f64) + t359;
    (t500, t503, t504, t506)
}
