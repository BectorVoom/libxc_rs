//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 976/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk976<F: Float>(t10930: F, t10931: F, t14364: F, t1890: F, t1966: F, t45690: F, t45693: F, t45700: F, t45703: F, t45711: F, t45713: F, t45717: F, t45723: F, t45725: F, t45729: F, t45731: F, t45735: F, t45736: F, t47402: F, t47403: F, t47405: F, t47406: F, t50111: F, t50130: F, t590: F, t7584: F, t7585: F) -> F {
    let t50253 = -t45690 - t45693 - F::new(0.51123901271894332902e0) * t1966 * t1890 * t14364 * t590 - t47402 - t47403 + t47405 + t47406 + t45700 - F::new(0.23005755572352449806e2) * t7584 * t7585 * t50111 + F::new(0.55213813373645879536e2) * t10930 * t10931 * t50130 + t45703 + t45711 + t45713 - t45717 - F::new(0.12780975317973583225e1) * t45723 - F::new(0.38342925953920749676e0) * t45725 + F::new(0.85206502119823888169e-1) * t45729 - t45731 - t45735 + t45736;
    t50253
}
