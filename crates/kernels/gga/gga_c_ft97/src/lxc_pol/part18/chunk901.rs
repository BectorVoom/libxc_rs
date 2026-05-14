//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 901/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk901<F: Float>(t22632: F, t5813: F, t5814: F, t1701: F, t2059: F, t5546: F, t22652: F, t554: F, t2030: F, t139: F, t39: F, t527: F, t2035: F, t538: F, t5790: F, t22803: F, t5838: F) -> (F, F, F, F, F, F, F, F) {
    let t23789 = t5813 * t22632 * t5814;
    let t23792 = t1701 * t5546 * t2059;
    let t23796 = t1701 * t22652 * t554;
    let t23806 = t1701 * t5546 * t2030;
    let t23809 = t139 * t39;
    let t23810 = t527 * t23809;
    let t23812 = t2035 * t5790 * t538;
    let t23817 = t5838 * t22803;
    (t23789, t23792, t23796, t23806, t23809, t23810, t23812, t23817)
}
