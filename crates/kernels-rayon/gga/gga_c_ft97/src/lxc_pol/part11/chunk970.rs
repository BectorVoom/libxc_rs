//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 970/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk970(t128: f64, t8153: f64, t8157: f64, t1995: f64, t8832: f64, t135: f64, t138: f64, t139: f64, t140: f64, t1683: f64, t1691: f64, t1698: f64, t1993: f64, t2031: f64, t2036: f64, t2037: f64, t2072: f64, t37903: f64, t37905: f64, t37931: f64, t40084: f64, t539: f64, t543: f64, t8812: f64, t8895: f64, t8935: f64) -> f64 {
    let t40206 = t128 * t8153 * t8157;
    let t40223 = t1995 * t8832;
    let t40226 = -0.45910941751869106328e2_f64 * t1993 * t1683 - 0.61919070671564293155e1_f64 * t8935 * t37931 * t135 * t138 + 0.22341601828860387373e3_f64 * t2036 * t1691 * t37903 * t139 + 0.1303559382559248277e1_f64 * t40206 * t539 + 0.87582322958871935983e1_f64 * t8812 * t2037 * t2031 - 0.35032929183548774394e2_f64 * t8895 * t1698 + 0.17516464591774387197e2_f64 * t1993 * t1698 + 0.44683203657720774746e3_f64 * t140 * t37905 - 0.43791161479435967991e1_f64 * t2036 * t2037 * t2072 - 0.89366407315441549491e3_f64 * t543 * t37905 - 0.28996384264338382944e2_f64 * t40223 * t40084;
    t40226
}
