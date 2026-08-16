//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 965/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk965(t49989: f64, t7290: f64, t1: f64, t106: f64, t14364: f64, t316: f64, t14358: f64, t1445: f64, t1890: f64, t1966: f64, t2087: f64, t2979: f64, t3720: f64, t3733: f64, t39050: f64, t45197: f64, t45199: f64, t45200: f64, t45212: f64, t45215: f64, t45217: f64, t45219: f64, t45222: f64, t45226: f64, t47149: f64, t47151: f64, t47166: f64, t47170: f64, t4820: f64, t5782: f64, t590: f64, t7513: f64, t780: f64, t8483: f64, t8634: f64) -> (f64, f64) {
    let t50002 = t7290 * t49989;
    let t50014 = t14364 * t1 * t106 * t316;
    let t50029 = -0.17041300423964777634e0_f64 * t47149 - 0.76685851907841499354e0_f64 * t47151 + 0.38342925953920749676e0_f64 * t45197 - t45199 - t45200 - 0.15889106645266856298e0_f64 * t7513 * t4820 * t50002 + 0.23833659967900284447e0_f64 * t39050 * t2979 + 0.38342925953920749677e1_f64 * t47166 + t45212 + t45215 + t45217 - 0.76685851907841499352e0_f64 * t45219 - 0.23005755572352449806e1_f64 * t47170 - 0.76685851907841499352e0_f64 * t45222 + 0.35750489951850426669e0_f64 * t780 * t50014 + t45226 + 0.71500979903700853338e0_f64 * t3733 * t8634 - 0.51123901271894332902e1_f64 * t1966 * t1890 * t49989 * t590 - 0.13803453343411469884e2_f64 * t5782 * t14358 - 0.13803453343411469884e2_f64 * t2087 * t1445 * t8483 * t3720;
    (t50002, t50029)
}
