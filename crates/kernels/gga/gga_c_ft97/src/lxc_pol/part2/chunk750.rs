//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 750/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk750<F: Float>(t13320: F, t3910: F, t1091: F, t2459: F, t2493: F, t1775: F, t3914: F, t2372: F, t3930: F, t1148: F, t8282: F, t3932: F, t11717: F, t3922: F, t3936: F, t458: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13321 = t3910 * t13320;
    let t13324 = t1091 * t2459;
    let t13325 = t2493 * t13324;
    let t13329 = 2.0 / 9.0 * t1775 * t3914;
    let t13332 = t2372 * t3930 * t2459;
    let t13335 = t8282 * t1148;
    let t13338 = 4.0 / 3.0 * t1775 * t3932;
    let t13339 = t11717 * t3922;
    let t13345 = 2.0 / 3.0 * t458 * t3936;
    (t13321, t13324, t13325, t13329, t13332, t13335, t13338, t13339, t13345)
}
