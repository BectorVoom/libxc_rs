//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 902/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk902<F: Float>(t665: F, t7640: F, t2344: F, t2680: F, t10491: F, t863: F, t192: F, t33828: F, t10696: F, t2749: F, t2770: F, t2843: F) -> (F, F, F, F, F, F, F) {
    let t43912 = t665 * t7640;
    let t43917 = t2344 * t2680;
    let t44030 = t10491 * t863;
    let t44280 = t192 * t33828;
    let t44351 = t863 * t10696;
    let t44369 = t2770 * t2749;
    let t44523 = t2770 * t2843;
    (t43912, t43917, t44030, t44280, t44351, t44369, t44523)
}
