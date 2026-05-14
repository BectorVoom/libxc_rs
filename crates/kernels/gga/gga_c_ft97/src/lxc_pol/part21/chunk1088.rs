//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1088/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1088<F: Float>(t51: F, t5827: F, t23831: F, t3379: F, t422: F, t12374: F, t23714: F, t5829: F, t6608: F, t92557: F, t2001: F, t94507: F, t1008: F, t358: F, t3404: F, t100519: F, t23711: F) -> (F, F, F, F, F, F, F, F, F) {
    let t104689 = t5827 * t51;
    let t104690 = t23831 * t104689;
    let t104695 = t422 * t3379;
    let t104704 = t12374 * t23714;
    let t104712 = t5829 * t92557 * t6608;
    let t104727 = t2001 * t94507;
    let t104735 = t1008 * t358;
    let t104782 = t422 * t3404;
    let t104788 = 0.26853068634149852184e-1 * t23711 * t100519;
    (t104689, t104690, t104695, t104704, t104712, t104727, t104735, t104782, t104788)
}
