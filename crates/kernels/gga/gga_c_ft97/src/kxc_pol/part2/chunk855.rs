//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 855/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk855<F: Float>(t13320: F, t3910: F, t1091: F, t2459: F, t2493: F, t1775: F, t3914: F, t2372: F, t3930: F, t1148: F, t8282: F, t3932: F) -> (F, F, F, F, F, F, F) {
    let t13321 = t3910 * t13320;
    let t13324 = t1091 * t2459;
    let t13325 = t2493 * t13324;
    let t13329 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1775 * t3914;
    let t13332 = t2372 * t3930 * t2459;
    let t13335 = t8282 * t1148;
    let t13338 = F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1775 * t3932;
    (t13321, t13324, t13325, t13329, t13332, t13335, t13338)
}
