//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 305/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk305<F: Float>(t4246: F, t875: F, t296: F, t1248: F, t2749: F, t824: F, t992: F) -> (F, F, F) {
    let t4247 = t4246 * t875;
    let t4248 = t296 * t4247;
    let t4251 = t2749 * t1248;
    let t4252 = t296 * t4251;
    let t4255 = t992 * t824;
    (t4248, t4252, t4255)
}
