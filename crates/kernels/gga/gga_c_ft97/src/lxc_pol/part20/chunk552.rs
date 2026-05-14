//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 552/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk552<F: Float>(t1775: F, t2503: F, t2489: F, t2508: F, t458: F, t9698: F, t259: F, t89: F, t9555: F, t2544: F, t681: F, t2399: F, t756: F, t2567: F, t754: F) -> (F, F, F, F, F, F, F, F) {
    let t9958 = t1775 * t2503;
    let t9960 = t1775 * t2489;
    let t9962 = t458 * t2508;
    let t9972 = 28.0 / 81.0 * t9698;
    let t9982 = 28.0 / 81.0 * t89 * t9555 * t259;
    let t9997 = t89 * t681 * t2544;
    let t10000 = t89 * t2399 * t756;
    let t10002 = t754 * t2567;
    (t9958, t9960, t9962, t9972, t9982, t9997, t10000, t10002)
}
