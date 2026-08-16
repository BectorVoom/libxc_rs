//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 764/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk764<F: Float>(t1013: F, t4056: F, t128: F, t2835: F, t2836: F, t4044: F, t4049: F, t4054: F, t408: F, t1023: F, t1505: F, t1044: F) -> (F, F, F, F, F, F) {
    let t4057 = t1013 * t4056;
    let t4058 = t128 * t4057;
    let t4060 = t2835 - F::cast_from(0.5936111111111111111e-2_f64) * t2836 - F::cast_from(0.5936111111111111111e-2_f64) * t4044 - F::cast_from(0.11872222222222222222e-1_f64) * t4049 + F::cast_from(0.35616666666666666666e-1_f64) * t4054 + F::cast_from(0.17808333333333333333e-1_f64) * t4058;
    let t4062 = F::cast_from(0.621814e-1_f64) * t4060 * t408;
    let t4063 = t1505 * t1023;
    let t4065 = F::cast_from(1.0_f64) * t4063 * t1044;
    (t4057, t4058, t4060, t4062, t4063, t4065)
}
