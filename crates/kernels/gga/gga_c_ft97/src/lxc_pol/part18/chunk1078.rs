//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1078/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1078<F: Float>(t23296: F, t8392: F, t1851: F, t5743: F, t23173: F, t1882: F, t23169: F, t23300: F, t23232: F, t5657: F, t8232: F, t5646: F, t23376: F, t23380: F, t23224: F, t23220: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t91796 = t8392 * t23296;
    let t91817 = t1851 * t5743;
    let t91862 = t8392 * t23173;
    let t91876 = t1882 * t23169;
    let t91881 = t8392 * t23300;
    let t91883 = t8392 * t23232;
    let t91895 = t8232 * t5657;
    let t91897 = t8232 * t5646;
    let t91899 = t1882 * t23376;
    let t91901 = t1882 * t23380;
    let t91903 = t1882 * t23224;
    let t91905 = t1882 * t23220;
    (t91796, t91817, t91862, t91876, t91881, t91883, t91895, t91897, t91899, t91901, t91903, t91905)
}
