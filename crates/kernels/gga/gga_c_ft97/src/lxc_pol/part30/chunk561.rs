//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 561/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk561<F: Float>(t232: F, t27557: F, t27561: F, t27596: F, t6014: F, t25: F, t6776: F, t3762: F, t1095: F, t24389: F, t13580: F, t1113: F, t202: F, t237: F, t17859: F, t24260: F) -> (F, F, F, F, F, F, F, F) {
    let t27686 = t232 * t27557;
    let t27689 = t232 * t27561;
    let t27692 = t6014 * t27596;
    let t27695 = t6776 * t25;
    let t27696 = t27695 * t3762;
    let t27699 = t24389 * t1095;
    let t27700 = t13580 * t27699;
    let t27703 = t202 * t1113;
    let t27704 = t27703 * t237;
    let t27707 = t24260 * t17859;
    (t27686, t27689, t27692, t27696, t27700, t27703, t27704, t27707)
}
