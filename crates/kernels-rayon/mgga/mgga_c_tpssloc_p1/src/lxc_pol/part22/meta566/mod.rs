//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta566 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2071;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2072;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta566(t10402: f64, t11037: f64, t2402: f64, t973: f64, t999: f64, t1030: f64, t10477: f64, t10472: f64, t10475: f64, t3128: f64, t10969: f64, t121: f64, t1043: f64, t204: f64, t1041: f64, t248: f64, t884: f64, t10337: f64, t964: f64, t340: f64, t625: f64, t221: f64, t339: f64, t344: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t42546, t42552, t42559, t42561, t42565, t42592) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2071(t10402, t11037, t2402, t973, t999, t1030, t10477, t10472, t10475, t3128, t10969, t121);
        let (t42749, t42752, t42811, t42813, t42817) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2072(t1043, t204, t1041, t248, t884, t10337, t964, t340, t625, t221, t339, t344);
    (t42546, t42552, t42559, t42561, t42565, t42592, t42749, t42752, t42811, t42813, t42817)
}
