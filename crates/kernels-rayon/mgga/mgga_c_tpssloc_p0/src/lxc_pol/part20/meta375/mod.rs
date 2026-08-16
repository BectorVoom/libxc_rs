//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta375 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1726;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1727;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1728;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta375(t1527: f64, t2719: f64, t10110: f64, t225: f64, t4143: f64, t2742: f64, t2718: f64, t4265: f64, t798: f64, t4145: f64, t4142: f64, t852: f64, t4300: f64, t865: f64, t2684: f64, t4180: f64, t4181: f64, t9646: f64, t9647: f64, t2633: f64, t2645: f64, t4248: f64, t1496: f64, t9541: f64, t12850: f64, t12860: f64, t12861: f64, t12889: f64, t12891: f64, t12894: f64, t12906: f64, t12910: f64, t9457: f64, t9462: f64, t9469: f64, t9476: f64, t9484: f64, t9496: f64, t9715: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13050, t13053, t13059, t13062, t13065, t13068) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1726(t1527, t2719, t10110, t225, t4143, t2742, t2718, t4265, t798, t4145, t4142, t852);
        let (t13072, t13076, t13080, t13084, t13087) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1727(t4300, t865, t2718, t2684, t4180, t4181, t9646, t9647, t2633, t2645, t4248, t1496, t9541);
        let t13093 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1728(t12850, t12860, t12861, t12889, t12891, t12894, t12906, t12910, t9457, t9462, t9469, t9476, t9484, t9496, t9715);
    (t13050, t13053, t13059, t13062, t13065, t13068, t13072, t13076, t13080, t13084, t13087, t13093)
}
