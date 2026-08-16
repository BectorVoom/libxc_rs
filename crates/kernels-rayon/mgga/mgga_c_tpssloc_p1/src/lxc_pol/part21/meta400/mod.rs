//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta400 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1878;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1879;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta400(t2906: f64, t4475: f64, t2932: f64, t4471: f64, t950: f64, t1581: f64, t1569: f64, t2862: f64, t10747: f64, t10771: f64, t10811: f64, t10825: f64, t10828: f64, t14429: f64, t14432: f64, t14436: f64, t14439: f64, t14443: f64, t14450: f64, t14453: f64, t2861: f64, t2886: f64, t2905: f64, t2930: f64, t4454: f64, t4476: f64, t14279: f64, t14373: f64, t14428: f64, t300: f64, t4446: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t14456, t14459, t14460, t14463, t14466, t14469) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1878(t2906, t4475, t2932, t4471, t950, t1581, t1569, t2862, t10747, t10771, t10811, t10825, t10828, t14429, t14432, t14436, t14439, t14443, t14450, t14453, t2861, t2886, t2905, t2930, t4454, t4476);
        let (t14472, t14473) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1879(t14279, t14373, t14428, t14469, t300, t4446);
    (t14456, t14459, t14460, t14463, t14466, t14472, t14473)
}
