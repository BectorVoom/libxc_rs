//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 923/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk923<F: Float>(t13891: F, t13950: F, t14041: F, t14091: F, t14153: F, t14209: F, t14251: F, t14292: F, t9735: F, t9701: F, t13746: F, t13753: F) -> (F, F, F, F, F) {
    let t14295 = t13891 + t13950 + t14041 + t14091 + t14153 + t14209 + t14251 + t14292;
    let t14317 = F::new(4.0) / F::new(81.0) * t9735;
    let t14318 = F::new(4.0) / F::new(27.0) * t9701;
    let t14327 = F::new(2.0) / F::new(9.0) * t13746;
    let t14329 = t13753 / F::new(9.0);
    (t14295, t14317, t14318, t14327, t14329)
}
