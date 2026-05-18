//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 965/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk965<F: Float>(t49989: F, t7290: F, t1: F, t106: F, t14364: F, t316: F, t14358: F, t1445: F, t1890: F, t1966: F, t2087: F, t2979: F, t3720: F, t3733: F, t39050: F, t45197: F, t45199: F, t45200: F, t45212: F, t45215: F, t45217: F, t45219: F, t45222: F, t45226: F, t47149: F, t47151: F, t47166: F, t47170: F, t4820: F, t5782: F, t590: F, t7513: F, t780: F, t8483: F, t8634: F) -> (F, F) {
    let t50002 = t7290 * t49989;
    let t50014 = t14364 * t1 * t106 * t316;
    let t50029 = -F::new(0.17041300423964777634e0) * t47149 - F::new(0.76685851907841499354e0) * t47151 + F::new(0.38342925953920749676e0) * t45197 - t45199 - t45200 - F::new(0.15889106645266856298e0) * t7513 * t4820 * t50002 + F::new(0.23833659967900284447e0) * t39050 * t2979 + F::new(0.38342925953920749677e1) * t47166 + t45212 + t45215 + t45217 - F::new(0.76685851907841499352e0) * t45219 - F::new(0.23005755572352449806e1) * t47170 - F::new(0.76685851907841499352e0) * t45222 + F::new(0.35750489951850426669e0) * t780 * t50014 + t45226 + F::new(0.71500979903700853338e0) * t3733 * t8634 - F::new(0.51123901271894332902e1) * t1966 * t1890 * t49989 * t590 - F::new(0.13803453343411469884e2) * t5782 * t14358 - F::new(0.13803453343411469884e2) * t2087 * t1445 * t8483 * t3720;
    (t50002, t50029)
}
