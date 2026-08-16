//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta375 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1629;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1630;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta375(t3131: f64, t4649: f64, t4593: f64, t4582: f64, t16558: f64, t998: f64, t974: f64, t13835: f64, t4531: f64, t13769: f64, t13839: f64, t1539: f64, t6733: f64, t4540: f64, t7577: f64, t4546: f64, t343: f64, t5842: f64, t984: f64, t2970: f64, t5824: f64, t973: f64, t10226: f64, t13782: f64, t13787: f64, t13790: f64, t13825: f64, t2960: f64, t2986: f64, t5825: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17732, t17733, t17734, t17737, t17738, t17742, t17745, t17748) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1629(t3131, t4649, t4593, t4582, t16558, t998, t974, t13835, t4531, t13769, t13839, t1539, t6733);
        let (t17752, t17757, t17763, t17764, t17766) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1630(t17748, t4531, t4540, t7577, t4546, t343, t5842, t984, t2970, t5824, t973, t10226, t13782, t13787, t13790, t13825, t17742, t17745, t2960, t2986, t5825);
    (t17732, t17733, t17734, t17737, t17738, t17748, t17752, t17757, t17763, t17764, t17766)
}
