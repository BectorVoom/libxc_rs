//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 411/1426 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk411<F: Float>(t1165: F, t1167: F, t1169: F, t1154: F, t1161: F, t14: F, t2063: F, t2067: F, t351: F, t740: F, t705: F, t78: F) -> (F, F) {
    let t2075 = -F::cast_from(0.99474444444444444447e-4_f64) * t1165 + F::cast_from(0.19894888888888888889e-3_f64) * t1167 + F::cast_from(0.52442777777777777777e-2_f64) * t1169;
    let t2078 = -t2063 * t1154 / F::new(18.0) - t2067 * t351 / F::new(6.0) + t740 * t1161 / F::new(9.0) + t14 * t2075 / F::new(2.0);
    let t2084 = t78 * t705;
    (t2078, t2084)
}
