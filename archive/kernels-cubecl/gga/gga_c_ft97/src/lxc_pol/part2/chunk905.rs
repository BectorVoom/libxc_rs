//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 905/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk905<F: Float>(t13811: F, t10119: F, t13798: F, t13801: F, t13804: F, t13807: F, t13814: F, t13817: F, t13820: F, t13823: F, t14004: F, t13977: F, t13986: F, t13999: F) -> F {
    let t14005 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t13811;
    let t14010 = -F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t13798 - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t13801 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t13804 + t13807 / F::cast_from(3.0_f64) - t14004 - t10119 - t14005 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t13814 - F::cast_from(2.0_f64) * t13817 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t13820 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t13823;
    let t14012 = t13977 + t13986 + t13999 + t14010;
    t14012
}
