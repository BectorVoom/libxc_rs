//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1056/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1056(t1005: f64, t20049: f64, t120: f64, t126: f64, t1631: f64, t16854: f64, t2007: f64, t2014: f64, t2021: f64, t20606: f64, t378: f64, t39877: f64, t39912: f64, t39926: f64, t39931: f64, t4417: f64, t4431: f64, t528: f64, t534: f64, t61854: f64, t61866: f64, t61889: f64, t61965: f64, t72: f64, t76945: f64, t76982: f64, t7914: f64, t85413: f64, t85424: f64, t85568: f64, t86741: f64, t86753: f64, t86756: f64, t86763: f64, t8948: f64, t8963: f64, t8965: f64, t8977: f64) -> f64 {
    let t86800 = t1005 * t20049;
    let t86824 = 0.32991033661753008702e-2_f64 * t39877 * t86753 - 0.532971647967385935e-1_f64 * t534 * t85568 * t126 + 0.41932428475884870816e-1_f64 * t1631 * t86756 + 0.43406294696984965172e-2_f64 * t8963 * t61889 * t61854 * t120 + 0.22136921132726965153e-3_f64 * t39926 * t76945 * t20606 - 0.8854768453090786061e-3_f64 * t8963 * t16854 * t8965 * t4431 - 0.11806357937454381415e-2_f64 * t8963 * t61866 * t39912 * t4417 + 0.17709536906181572122e-2_f64 * t8963 * t16854 * t39931 * t4417 - 0.30699166922921429856e0_f64 * t8977 * t86741 + 0.1279131955121726244e0_f64 * t2021 * t86800 - 0.15095674251318553494e0_f64 * t7914 * t86741 + 0.55909904634513161088e-1_f64 * t1631 * t86800 + t76982 - 0.79692916077817074549e-2_f64 * t2014 * t72 * t85424 * t120 - 0.49254336522043865661e-4_f64 * t8948 * t378 * t86763 + t61965 - 0.36171912247487470976e-3_f64 * t2014 * t72 * t85413 * t2007 * t120 - 0.90429780618718677442e-4_f64 * t8948 * t378 * t85413 * t528 * t120;
    t86824
}
