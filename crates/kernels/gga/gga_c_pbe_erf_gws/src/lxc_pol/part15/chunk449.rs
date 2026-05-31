//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 449/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk449<F: Float>(t1689: F, t1694: F, t1700: F, t1704: F, t1743: F, t203: F, t184: F, t221: F, t174: F, t177: F, t332: F, t395: F, t574: F) -> (F, F, F, F, F, F, F) {
    let t1748 = -t1743 - F::cast_from(0.12594444444444444445e-2_f64) * t1689 + F::cast_from(0.12594444444444444445e-2_f64) * t1694 - F::cast_from(0.37783333333333333334e-2_f64) * t1700 + F::cast_from(0.18891666666666666667e-2_f64) * t1704;
    let t1749 = t203 * t1748;
    let t1750 = t1749 * t184;
    let t1752 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t1750 * t221;
    let t1754 = t174 * t332 * t177;
    let t1755 = F::cast_from(0.25188888888888888889e-2_f64) * t1754;
    let t1756 = t395 * t574;
    (t1748, t1749, t1750, t1752, t1754, t1755, t1756)
}
