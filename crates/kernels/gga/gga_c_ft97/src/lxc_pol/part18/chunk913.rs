//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 913/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk913<F: Float>(t23900: F, t363: F, t1969: F, t446: F, t1580: F, t5916: F, t1359: F, t1557: F) -> (F, F, F, F, F) {
    let t23901 = t23900 * t363;
    let t23902 = t1969 * t23901;
    let t23903 = t446 * t23902;
    let t23905 = t5916 * t1580;
    let t23906 = t1969 * t23905;
    let t23907 = t446 * t23906;
    let t23909 = t1359 * t1557;
    (t23902, t23903, t23906, t23907, t23909)
}
