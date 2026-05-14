//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 581/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk581<F: Float>(t25943: F, t446: F, t1882: F, t6513: F, t23054: F, t6501: F, t22980: F, t22991: F, t23016: F, t23029: F, t23114: F, t25926: F, t25931: F, t25935: F, t25940: F, t358: F, t6454: F) -> (F, F, F, F, F) {
    let t25944 = t446 * t25943;
    let t25946 = t1882 * t6513;
    let t25948 = t23054 * t6501;
    let t25952 = -t25926 / 9.0 + t25931 / 27.0 - t25935 / 9.0 - t22980 / 9.0 - t22991 / 27.0 + t25940 / 9.0 + t25944 / 9.0 - t25946 / 27.0 - t25948 / 54.0 - t23016 / 36.0 + t23029 / 18.0 - t23114;
    let t25955 = t6454 * t358;
    (t25944, t25946, t25948, t25952, t25955)
}
