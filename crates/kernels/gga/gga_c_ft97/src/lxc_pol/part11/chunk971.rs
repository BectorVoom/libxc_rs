//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 971/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk971<F: Float>(t245: F, t10174: F, t1580: F, t21: F, t2624: F, t267: F, t363: F, t37391: F, t41988: F, t43018: F, t5: F, t7745: F, t776: F, t1934: F, t505: F, t904: F, t327: F, t41446: F) -> (F, F, F) {
    let t246 = 10000000.0 <= t245;
    let t43034 = piecewise3(t246, 0.0, t5 * (t41988 + t43018) * t21 / 4.0 + t5 * t10174 * t363 + 3.0 / 2.0 * t5 * t2624 * t1580 + t5 * t776 * t7745 + t5 * t267 * t37391 / 4.0);
    let t43046 = t1934 * t904 * t505;
    let t43050 = t327 * t41446;
    (t43034, t43046, t43050)
}
