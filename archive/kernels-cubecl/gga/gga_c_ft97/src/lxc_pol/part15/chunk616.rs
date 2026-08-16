//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 616/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk616<F: Float>(t167: F, t9114: F, t2179: F, t582: F, t2: F, t9952: F, t1148: F, t8282: F, t1609: F, t2378: F, t4952: F, t6783: F) -> (F, F, F, F, F, F) {
    let t13212 = t9114 * t167;
    let t13220 = t582 * t2179;
    let t13313 = t9952 * t2;
    let t13335 = t8282 * t1148;
    let t13411 = t1609 * t2378;
    let t13414 = t6783 * t4952;
    (t13212, t13220, t13313, t13335, t13411, t13414)
}
