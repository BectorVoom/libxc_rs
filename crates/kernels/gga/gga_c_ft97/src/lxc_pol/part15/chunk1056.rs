//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1056/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1056<F: Float>(t1005: F, t20049: F, t120: F, t126: F, t1631: F, t16854: F, t2007: F, t2014: F, t2021: F, t20606: F, t378: F, t39877: F, t39912: F, t39926: F, t39931: F, t4417: F, t4431: F, t528: F, t534: F, t61854: F, t61866: F, t61889: F, t61965: F, t72: F, t76945: F, t76982: F, t7914: F, t85413: F, t85424: F, t85568: F, t86741: F, t86753: F, t86756: F, t86763: F, t8948: F, t8963: F, t8965: F, t8977: F) -> F {
    let t86800 = t1005 * t20049;
    let t86824 = F::cast_from(0.32991033661753008702e-2_f64) * t39877 * t86753 - F::cast_from(0.532971647967385935e-1_f64) * t534 * t85568 * t126 + F::cast_from(0.41932428475884870816e-1_f64) * t1631 * t86756 + F::cast_from(0.43406294696984965172e-2_f64) * t8963 * t61889 * t61854 * t120 + F::cast_from(0.22136921132726965153e-3_f64) * t39926 * t76945 * t20606 - F::cast_from(0.8854768453090786061e-3_f64) * t8963 * t16854 * t8965 * t4431 - F::cast_from(0.11806357937454381415e-2_f64) * t8963 * t61866 * t39912 * t4417 + F::cast_from(0.17709536906181572122e-2_f64) * t8963 * t16854 * t39931 * t4417 - F::cast_from(0.30699166922921429856e0_f64) * t8977 * t86741 + F::cast_from(0.1279131955121726244e0_f64) * t2021 * t86800 - F::cast_from(0.15095674251318553494e0_f64) * t7914 * t86741 + F::cast_from(0.55909904634513161088e-1_f64) * t1631 * t86800 + t76982 - F::cast_from(0.79692916077817074549e-2_f64) * t2014 * t72 * t85424 * t120 - F::cast_from(0.49254336522043865661e-4_f64) * t8948 * t378 * t86763 + t61965 - F::cast_from(0.36171912247487470976e-3_f64) * t2014 * t72 * t85413 * t2007 * t120 - F::cast_from(0.90429780618718677442e-4_f64) * t8948 * t378 * t85413 * t528 * t120;
    t86824
}
