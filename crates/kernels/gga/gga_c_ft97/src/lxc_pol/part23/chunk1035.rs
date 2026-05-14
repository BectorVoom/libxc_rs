//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1035/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1035<F: Float>(t1196: F, t2035: F, t6979: F, t1701: F, t5260: F, t6027: F, t5014: F, t6789: F) -> (F, F, F) {
    let t31502 = t2035 * t6979 * t1196;
    let t31508 = t1701 * t6027 * t5260;
    let t31515 = t6789 * t5014;
    (t31502, t31508, t31515)
}
