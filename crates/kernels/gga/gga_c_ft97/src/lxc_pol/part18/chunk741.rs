//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 741/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk741<F: Float>(t12334: F, t12703: F, t12666: F, t12670: F, t12672: F, t12674: F, t12676: F, t12677: F, t12681: F, t12685: F, t12689: F, t12696: F, t12700: F, t1901: F, t446: F, t9090: F, t9097: F, t9106: F) -> (F,) {
    let t12704 = t12703 * t12334;
    let t12707 = 2.0 / 3.0 * t446 * t12666 + t12670 + t12672 + t12674 + t12676 + 2.0 / 9.0 * t1901 * t12677 + 2.0 / 9.0 * t1901 * t12681 - t446 * t12685 / 3.0 - t446 * t12689 / 3.0 - 2.0 / 27.0 * t9090 - 2.0 / 27.0 * t9097 + t9106 / 9.0 - t446 * t12696 / 3.0 - 2.0 / 3.0 * t446 * t12700 - 4.0 / 9.0 * t1901 * t12704;
    (t12707,)
}
