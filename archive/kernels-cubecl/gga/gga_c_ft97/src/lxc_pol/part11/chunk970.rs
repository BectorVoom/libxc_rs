//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 970/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk970<F: Float>(t128: F, t8153: F, t8157: F, t1995: F, t8832: F, t135: F, t138: F, t139: F, t140: F, t1683: F, t1691: F, t1698: F, t1993: F, t2031: F, t2036: F, t2037: F, t2072: F, t37903: F, t37905: F, t37931: F, t40084: F, t539: F, t543: F, t8812: F, t8895: F, t8935: F) -> F {
    let t40206 = t128 * t8153 * t8157;
    let t40223 = t1995 * t8832;
    let t40226 = -F::cast_from(0.45910941751869106328e2_f64) * t1993 * t1683 - F::cast_from(0.61919070671564293155e1_f64) * t8935 * t37931 * t135 * t138 + F::cast_from(0.22341601828860387373e3_f64) * t2036 * t1691 * t37903 * t139 + F::cast_from(0.1303559382559248277e1_f64) * t40206 * t539 + F::cast_from(0.87582322958871935983e1_f64) * t8812 * t2037 * t2031 - F::cast_from(0.35032929183548774394e2_f64) * t8895 * t1698 + F::cast_from(0.17516464591774387197e2_f64) * t1993 * t1698 + F::cast_from(0.44683203657720774746e3_f64) * t140 * t37905 - F::cast_from(0.43791161479435967991e1_f64) * t2036 * t2037 * t2072 - F::cast_from(0.89366407315441549491e3_f64) * t543 * t37905 - F::cast_from(0.28996384264338382944e2_f64) * t40223 * t40084;
    t40226
}
