//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta692 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2507;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2508;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta692(t13176: f64, t2696: f64, t849: f64, t13360: f64, t2707: f64, t1509: f64, t9975: f64, t242: f64, t41347: f64, t812: f64, t13297: f64, t9573: f64, t13080: f64, t9638: f64, t226: f64, t40931: f64, t68: f64, t13377: f64, t814: f64, t13396: f64, t808: f64, t13068: f64, t225: f64, t13030: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t47278, t47279, t47283, t47285, t47307, t47333) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2507(t13176, t2696, t849, t13360, t2707, t1509, t9975, t242, t41347, t812, t13297, t9573);
        let (t47353, t47386, t47395, t47419, t47568, t47585) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2508(t13080, t9638, t226, t40931, t68, t13377, t814, t13396, t808, t13068, t225, t13030);
    (t47278, t47279, t47283, t47285, t47307, t47333, t47353, t47386, t47395, t47419, t47568, t47585)
}
