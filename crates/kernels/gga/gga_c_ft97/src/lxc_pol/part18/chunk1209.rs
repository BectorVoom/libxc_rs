//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1209/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1209<F: Float>(t1317: F, t1637: F, t6504: F, t25997: F, t376: F, t23057: F, t25893: F, t25894: F, t452: F, t1871: F, t22952: F, t3157: F, t432: F, t5675: F, t473: F, t23054: F, t25901: F) -> (F, F, F, F, F, F, F) {
    let t101879 = t1317 * t1637 * t6504;
    let t101882 = t1317 * t376 * t25997;
    let t101883 = 2.0 / 9.0 * t101882;
    let t101886 = t25893 * t452 * t23057 * t25894;
    let t101891 = t22952 * t1871 * t5675 * t3157 * t432;
    let t101896 = t25893 * t452 * t5675 * t3157 * t473;
    let t101898 = t23054 * t25901;
    (t101879, t101882, t101883, t101886, t101891, t101896, t101898)
}
