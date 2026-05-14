//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1232/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1232<F: Float>(t17806: F, t17836: F, t108581: F, t6034: F, t6804: F, t108530: F, t4075: F, t992: F, t111831: F, t505: F, t1091: F, t3762: F, t27501: F, t27642: F, t122737: F, t70: F) -> (F, F, F, F, F, F, F) {
    let t123607 = t17836 * t17806;
    let t123612 = t6034 * t108581 * t6804;
    let t123615 = t4075 * t108530 * t992;
    let t123619 = t111831 * t505;
    let t123650 = t1091 * t3762;
    let t123672 = t27642 * t27501;
    let t123675 = t122737 * t70;
    (t123607, t123612, t123615, t123619, t123650, t123672, t123675)
}
