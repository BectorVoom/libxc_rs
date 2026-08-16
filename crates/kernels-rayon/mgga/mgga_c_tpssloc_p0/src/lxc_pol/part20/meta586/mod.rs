//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta586 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2156;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2157;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2158;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2159;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta586(t23508: f64, t43292: f64, t11013: f64, t225: f64, t10163: f64, t386: f64, t68: f64, t11008: f64, t3215: f64, t3399: f64, t3402: f64, t11176: f64, t300: f64, t3639: f64, t11160: f64, t690: f64, t11169: f64, t2394: f64, t3244: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t43577, t43599, t43604, t43619, t43637, t43689, t43692, t43700) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2156(t23508, t43292, t11013, t225, t10163, t386, t68, t11008, t3215, t3399, t3402, t11176, t300);
        let (t43706, t43727) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2157(t3639, t11160, t690);
        let t43729 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2158(t11169, t690);
        let t43748 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2159(t2394, t3244);
    (t43577, t43599, t43604, t43619, t43637, t43689, t43692, t43700, t43706, t43727, t43729, t43748)
}
