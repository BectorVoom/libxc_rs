//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta500 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2009;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta500(t1814: f64, t5333: f64, t1819: f64, t68: f64, t3792: f64, t5286: f64, t5343: f64, t5234: f64, t5245: f64, t576: f64, t671: f64, t3701: f64, t3914: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t19654, t19708, t19735, t19810, t19876, t20173, t22578) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2009(t1814, t5333, t1819, t68, t3792, t5286, t5343, t5234, t5245, t576, t671, t3701, t3914);
    (t19654, t19708, t19735, t19810, t19876, t20173, t22578)
}
