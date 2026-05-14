//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 730/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk730<F: Float>(t10508: F, t10672: F, t10734: F, t10806: F, t10660: F, t312: F, t10236: F, t10238: F, t10432: F, t10667: F, t10680: F, t10689: F, t10699: F, t10713: F, t10800: F, t2649: F, t2745: F, t2892: F, t301: F, t317: F, t830: F, t880: F) -> (F, F, F) {
    let t10808 = t10508 + t10672 + t10734 + t10806;
    let t10810 = t10660 * t312;
    let t10818 = -t10236 * t317 - 2.0 * t10238 * t317 - t10432 * t317 - t10808 * t301 - 3.0 * t2649 * t880 - 3.0 * t2745 * t880 - 3.0 * t2892 * t830 - 6.0 * t10667 - 6.0 * t10680 + 12.0 * t10689 - 12.0 * t10699 + 12.0 * t10713 - 2.0 * t10800 + 2.0 * t10810;
    (t10808, t10810, t10818)
}
