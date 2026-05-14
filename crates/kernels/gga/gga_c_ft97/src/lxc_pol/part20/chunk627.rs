//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 627/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk627<F: Float>(t13830: F, t766: F, t242: F, t1175: F, t2459: F, t729: F, t1160: F, t737: F) -> (F, F, F) {
    let t13831 = t13830 * t766;
    let t13832 = t242 * t13831;
    let t13836 = t729 * t1175 * t2459;
    let t13839 = t737 * t1160;
    (t13832, t13836, t13839)
}
