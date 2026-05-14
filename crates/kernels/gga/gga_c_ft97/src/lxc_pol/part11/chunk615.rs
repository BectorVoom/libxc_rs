//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 615/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk615<F: Float>(t1969: F, t9078: F, t446: F, t9039: F, t9043: F, t9047: F, t9052: F, t9057: F, t9059: F, t9062: F, t9065: F, t9068: F, t9072: F, t9076: F, t9037: F, t515: F) -> (F, F, F, F) {
    let t9079 = t1969 * t9078;
    let t9080 = t446 * t9079;
    let t9082 = -t9039 / 9.0 + t9043 / 6.0 + t9047 / 6.0 + t9052 / 9.0 + 2.0 / 9.0 * t9057 - t9059 / 9.0 - t9062 / 9.0 - 2.0 / 9.0 * t9065 + t9068 / 6.0 - t9072 - t9076 / 3.0 - t9080 / 3.0;
    let t9083 = t9037 + t9082;
    let t9084 = t515 * t9083;
    (t9079, t9080, t9083, t9084)
}
