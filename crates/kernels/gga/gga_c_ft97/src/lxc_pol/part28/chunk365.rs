//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 365/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk365<F: Float>(t1307: F, t473: F, t469: F, t28: F, t5665: F, t1317: F, t1318: F, t376: F, t1316: F, t92: F) -> (F, F, F, F, F) {
    let t5666 = t1307 * t473;
    let t5667 = t469 * t5666;
    let t5669 = t5665 * t28 * t5667;
    let t5672 = t1317 * t376 * t1318;
    let t5673 = t5672 / F::cast_from(18.0_f64);
    let t5674 = t1316 * t92;
    (t5667, t5669, t5672, t5673, t5674)
}
