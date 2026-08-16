//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta334 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1623;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1624;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta334(t1176: f64, t3242: f64, t9288: f64, t974: f64, t11638: f64, t475: f64, t1214: f64, t248: f64, t11616: f64, t68: f64, t484: f64, t10913: f64, t4972: f64, t4582: f64, t1174: f64, t11821: f64, t11825: f64, t11834: f64, t11836: f64, t11839: f64, t11842: f64, t11845: f64, t1213: f64, t1227: f64, t1232: f64, t3490: f64, t3527: f64, t3531: f64, t3587: f64, t488: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11849, t11850, t11853, t11855, t11858, t11859, t11862) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1623(t1176, t3242, t9288, t974, t11638, t475, t1214, t248, t11616, t68, t484, t10913, t4972);
        let (t11863, t11866) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1624(t11862, t4582, t1174, t11821, t11825, t11834, t11836, t11839, t11842, t11845, t11850, t11855, t11859, t1213, t1227, t1232, t3490, t3527, t3531, t3587, t488);
    (t11849, t11850, t11853, t11855, t11858, t11859, t11862, t11863, t11866)
}
