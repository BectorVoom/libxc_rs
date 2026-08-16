//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 976/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk976(t10930: f64, t10931: f64, t14364: f64, t1890: f64, t1966: f64, t45690: f64, t45693: f64, t45700: f64, t45703: f64, t45711: f64, t45713: f64, t45717: f64, t45723: f64, t45725: f64, t45729: f64, t45731: f64, t45735: f64, t45736: f64, t47402: f64, t47403: f64, t47405: f64, t47406: f64, t50111: f64, t50130: f64, t590: f64, t7584: f64, t7585: f64) -> f64 {
    let t50253 = -t45690 - t45693 - 0.51123901271894332902e0_f64 * t1966 * t1890 * t14364 * t590 - t47402 - t47403 + t47405 + t47406 + t45700 - 0.23005755572352449806e2_f64 * t7584 * t7585 * t50111 + 0.55213813373645879536e2_f64 * t10930 * t10931 * t50130 + t45703 + t45711 + t45713 - t45717 - 0.12780975317973583225e1_f64 * t45723 - 0.38342925953920749676e0_f64 * t45725 + 0.85206502119823888169e-1_f64 * t45729 - t45731 - t45735 + t45736;
    t50253
}
