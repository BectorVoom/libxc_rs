//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 255/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk255<F: Float>(t1730: F, t152: F, t153: F, t151: F, t1771: F, t1775: F, t583: F, t458: F, t588: F, t143: F, t1554: F) -> (F, F, F, F, F, F) {
    let t2066 = F::cast_from(0.11113000182098765433e-1_f64) * t1730;
    let t2086 = F::cast_from(1.0_f64) / t153 / t152;
    let t2092 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1771 * t151;
    let t2093 = t1775 * t583;
    let t2095 = t458 * t588;
    let t2097 = t1554 * t143;
    (t2066, t2086, t2092, t2093, t2095, t2097)
}
