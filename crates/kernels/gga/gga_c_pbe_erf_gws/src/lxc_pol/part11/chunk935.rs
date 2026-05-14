//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 935/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk935<F: Float>(t12041: F, t37701: F, t3854: F, t1105: F, t3855: F, t2407: F, t858: F, t1134: F, t3717: F, t3825: F, t3128: F, t36837: F, t13357: F, t6203: F, t13296: F, t20944: F) -> (F, F, F, F, F, F, F, F, F) {
    let t45088 = t12041 * t37701;
    let t45100 = t3854 * param_a_c;
    let t45126 = t3855 * t1105;
    let t45128 = t2407 * t858 * t45126;
    let t45133 = t2407 * t858 * t1134 * t3717;
    let t45140 = t2407 * t858 * t3825 * t1105;
    let t45190 = t3128 * t36837;
    let t45192 = t6203 * t13357;
    let t45194 = t20944 * t13296;
    (t45088, t45100, t45126, t45128, t45133, t45140, t45190, t45192, t45194)
}
