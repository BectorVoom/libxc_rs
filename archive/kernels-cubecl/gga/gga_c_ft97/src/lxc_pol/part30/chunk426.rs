//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 426/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk426<F: Float>(t291: F, t6979: F, t1208: F, t1701: F, t6027: F, t1196: F, t231: F, t6045: F, t1200: F, t287: F) -> (F, F, F, F, F) {
    let t6980 = t6979 * t291;
    let t6986 = t1701 * t6027 * t1208;
    let t6999 = t231 * t1196;
    let t7000 = t6045 * t6999;
    let t7003 = t1200 * t287;
    (t6980, t6986, t6999, t7000, t7003)
}
