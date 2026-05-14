//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 637/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk637<F: Float>(t1775: F, t3911: F, t2: F, t9952: F, t3914: F, t1148: F, t8282: F, t3932: F, t11717: F, t3922: F, t3936: F, t458: F, t3927: F, t1609: F, t2378: F, t2427: F, t6: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t13308 = 4.0 / 27.0 * t1775 * t3911;
    let t13313 = t9952 * t2;
    let t13329 = 2.0 / 9.0 * t1775 * t3914;
    let t13335 = t8282 * t1148;
    let t13338 = 4.0 / 3.0 * t1775 * t3932;
    let t13339 = t11717 * t3922;
    let t13345 = 2.0 / 3.0 * t458 * t3936;
    let t13388 = 2.0 / 9.0 * t1775 * t3927;
    let t13411 = t1609 * t2378;
    let t13442 = t2427 * t6;
    (t13308, t13313, t13329, t13335, t13338, t13339, t13345, t13388, t13411, t13442)
}
