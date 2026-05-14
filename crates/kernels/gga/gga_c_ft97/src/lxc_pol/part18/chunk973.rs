//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 973/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk973<F: Float>(t22980: F, t22991: F, t23016: F, t23029: F, t23038: F, t25926: F, t25931: F, t25935: F, t25940: F, t25944: F, t25946: F, t25948: F, t23048: F, t23055: F, t25958: F, t25962: F, t25966: F, t25970: F, t25973: F, t25976: F, t25979: F, t25983: F, t25988: F, t25993: F) -> (F, F) {
    let t26089 = -t25926 / 3.0 + t25931 / 9.0 - t25935 / 3.0 - t22980 / 3.0 - t22991 / 9.0 + t25940 / 3.0 + t25944 / 3.0 - t25946 / 9.0 - t25948 / 18.0 - t23016 / 12.0 + t23029 / 6.0 - t23038;
    let t26102 = -2.0 / 3.0 * t23048 + t25958 / 3.0 + 2.0 / 3.0 * t25962 - 6.0 * t25966 - t23055 / 18.0 - 2.0 / 3.0 * t25970 - 2.0 / 3.0 * t25973 - 2.0 / 3.0 * t25976 + 2.0 / 9.0 * t25979 - t25983 / 12.0 - t25988 / 12.0 + t25993;
    (t26089, t26102)
}
