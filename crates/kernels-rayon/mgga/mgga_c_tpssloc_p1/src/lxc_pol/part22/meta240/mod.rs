//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta240 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1327;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1328;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1329;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta240(t252: f64, t9971: f64, t856: f64, t68: f64, t261: f64, t2751: f64, t1053: f64, t1887: f64, t337: f64, t615: f64, t134: f64, t976: f64, t984: f64, t271: f64, t2775: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10080, t10108, t10109, t10110, t10143, t10163, t10164, t10165, t10186) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1327(t252, t9971, t856, t68, t261, t2751, t1053, t1887, t337, t615);
        let t10189 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1328(t134, t976);
        let (t10190, t10213) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1329(t10189, t984, t271, t2775);
    (t10080, t10108, t10109, t10110, t10143, t10163, t10164, t10165, t10186, t10189, t10190, t10213)
}
