//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta350 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1560;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1561;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta350(t16935: f64, t4180: f64, t4181: f64, t2639: f64, t5619: f64, t5614: f64, t1484: f64, t4119: f64, t2701: f64, t820: f64, t5544: f64, t776: f64, t2697: f64, t5628: f64, t210: f64, t5567: f64, t1495: f64, t5571: f64, t13223: f64, t5591: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16937, t16940, t16942, t16944, t16946, t16949) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1560(t16935, t4180, t4181, t2639, t5619, t5614, t1484, t4119, t2701, t820, t5544, t776);
        let (t16951, t16954, t16957, t16961, t16965, t16968) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1561(t16949, t2701, t820, t2697, t5628, t210, t5567, t776, t1495, t4119, t5571, t13223, t5591);
    (t16937, t16940, t16942, t16944, t16946, t16949, t16951, t16954, t16957, t16961, t16965, t16968)
}
