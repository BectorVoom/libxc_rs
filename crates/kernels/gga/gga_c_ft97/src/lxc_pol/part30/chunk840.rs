//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 840/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk840<F: Float>(t35516: F, t743: F, t1434: F, t193: F, t33460: F, t9770: F, t992: F, t446: F, t1131: F, t33243: F, t89: F, t6008: F, t6837: F) -> (F, F, F, F, F, F, F) {
    let t35517 = t743 * t35516;
    let t35519 = t1434 * t193 * t35517;
    let t35522 = t9770 * t33460 * t992;
    let t35523 = t446 * t35522;
    let t35525 = t33243 * t1131;
    let t35526 = t193 * t35525;
    let t35527 = t89 * t35526;
    let t35529 = t6008 * t6837;
    (t35517, t35519, t35522, t35523, t35525, t35527, t35529)
}
