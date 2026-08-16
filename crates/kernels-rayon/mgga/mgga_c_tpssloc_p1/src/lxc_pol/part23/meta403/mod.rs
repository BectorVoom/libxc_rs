//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta403 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1213;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1214;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta403(t13965: f64, t4641: f64, t1020: f64, t10508: f64, t248: f64, t5867: f64, t3039: f64, t5878: f64, t14202: f64, t4644: f64, t3082: f64, t5905: f64, t1041: f64, t43338: f64, t5677: f64, t3070: f64, t43198: f64, t5908: f64, t5884: f64, t698: f64, t973: f64, t5889: f64, t5893: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t62148, t62177, t62183, t62284, t62360) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1213(t13965, t4641, t1020, t10508, t248, t5867, t3039, t5878, t14202, t4644, t3082, t5905);
        let (t62445, t62494, t62559, t62565, t62832) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1214(t1041, t248, t43338, t5677, t3070, t43198, t5908, t5884, t698, t973, t5889, t5893);
    (t62148, t62177, t62183, t62284, t62360, t62445, t62494, t62559, t62565, t62832)
}
