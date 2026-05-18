//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 671/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk671<F: Float>(t9698: F, t259: F, t89: F, t9555: F, t2399: F, t756: F, t2567: F, t754: F, t2492: F, t762: F, t241: F, t9568: F) -> (F, F, F, F, F, F) {
    let t9972 = F::new(28.0) / F::new(81.0) * t9698;
    let t9982 = F::new(28.0) / F::new(81.0) * t89 * t9555 * t259;
    let t10000 = t89 * t2399 * t756;
    let t10002 = t754 * t2567;
    let t10007 = t2492 * t762;
    let t10024 = t9568 * t241;
    (t9972, t9982, t10000, t10002, t10007, t10024)
}
