//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 911/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk911<F: Float>(t14075: F, t3885: F, t2606: F, t258: F, t9577: F, t13863: F, t3891: F, t2459: F, t992: F, t2600: F, t2599: F, t3972: F, t766: F) -> (F, F, F, F) {
    let t14094 = t3885 * t14075;
    let t14095 = t2606 * t14094;
    let t14098 = t258 * t9577;
    let t14099 = t14098 * t13863;
    let t14100 = t3891 * t14099;
    let t14103 = t992 * t2459;
    let t14104 = t2600 * t14103;
    let t14105 = t2599 * t14104;
    let t14108 = t3972 * t766;
    (t14095, t14100, t14105, t14108)
}
