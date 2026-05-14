//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 801/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk801<F: Float>(t22643: F, t5821: F, t22632: F, t5813: F, t5814: F, t139: F, t39: F, t527: F, t22803: F, t5838: F, t2057: F, t5585: F, t3392: F, t135: F, t1995: F, t5820: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t23781 = t5821 * t22643;
    let t23789 = t5813 * t22632 * t5814;
    let t23809 = t139 * t39;
    let t23810 = t527 * t23809;
    let t23817 = t5838 * t22803;
    let t23823 = t2057 * t39;
    let t23824 = t23823 * t5585;
    let t23825 = t3392 * t23824;
    let t23831 = t1995 * t135;
    let t23832 = t23831 * t5820;
    (t23781, t23789, t23809, t23810, t23817, t23823, t23824, t23825, t23831, t23832)
}
