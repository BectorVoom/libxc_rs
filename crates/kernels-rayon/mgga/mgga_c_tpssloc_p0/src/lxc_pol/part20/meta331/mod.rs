//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta331 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1617;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1618;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta331(t1090: f64, t11789: f64, t248: f64, t1227: f64, t3536: f64, t3572: f64, t3252: f64, t3521: f64, t3248: f64, t11172: f64, t1230: f64, t11163: f64, t1009: f64, t3481: f64, t1011: f64, t1212: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11791, t11792, t11794, t11797, t11798, t11801, t11802, t11805, t11809) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1617(t1090, t11789, t248, t1227, t3536, t3572, t3252, t3521, t3248, t11172, t1230, t11163);
        let (t11812, t11813, t11814) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1618(t1009, t3481, t1011, t1212);
    (t11791, t11792, t11794, t11797, t11798, t11801, t11802, t11805, t11809, t11812, t11813, t11814)
}
