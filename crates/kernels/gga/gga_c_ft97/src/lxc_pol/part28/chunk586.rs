//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 586/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk586<F: Float>(t25846: F, t370: F, t27: F, t89: F, t469: F, t1317: F, t28: F, t376: F, t6508: F, t23081: F, t23124: F, t25999: F, t26004: F, t26009: F, t26014: F, t26019: F, t26022: F, t26025: F) -> (F, F, F, F, F) {
    let t26027 = t370 * t25846;
    let t26029 = t89 * t27 * t26027;
    let t26031 = t469 * t25846;
    let t26033 = t1317 * t28 * t26031;
    let t26036 = t1317 * t376 * t6508;
    let t26039 = t25999 / 3.0 + t26004 / 3.0 + t26009 / 12.0 + t26014 / 12.0 + t26019 / 12.0 - t26022 / 9.0 - t26025 / 36.0 - t26029 / 3.0 - t26033 / 6.0 + t26036 / 18.0 - t23124 + t23081 / 9.0;
    (t26027, t26029, t26033, t26036, t26039)
}
