//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta496 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1924;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1925;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1926;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta496(t10377: f64, t10385: f64, t10480: f64, t10876: f64, t10883: f64, t14508: f64, t14511: f64, t17612: f64, t17616: f64, t21393: f64, t21398: f64, t21405: f64, t21483: f64, t21487: f64, t21490: f64, t21493: f64, t3130: f64, t378: f64, t5875: f64, t5880: f64, t973: f64, t1616: f64, t17712: f64, t4582: f64, t1409: f64, t5398: f64, t4588: f64, t10970: f64, t21130: f64, t248: f64, t5681: f64, t3071: f64, t1539: f64, t5873: f64, t10403: f64, t1041: f64, t13966: f64, t13995: f64, t17621: f64, t17625: f64, t17656: f64, t17660: f64, t17662: f64, t17668: f64, t3039: f64, t3070: f64, t5909: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t21498 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1924(t10377, t10385, t10480, t10876, t10883, t14508, t14511, t17612, t17616, t21393, t21398, t21405, t21483, t21487, t21490, t21493, t3130, t378, t5875, t5880, t973);
        let (t21502, t21503, t21510) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1925(t1616, t17712, t4582, t1409, t5398);
        let (t21511, t21512, t21516, t21519, t21520, t21525, t21526, t21529) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1926(t21510, t4588, t4582, t10970, t21130, t248, t1616, t5681, t3071, t1539, t5873, t10403, t1041, t13966, t13995, t17621, t17625, t17656, t17660, t17662, t17668, t21503, t3039, t3070, t5909);
    (t21498, t21502, t21503, t21510, t21511, t21512, t21516, t21519, t21520, t21525, t21526, t21529)
}
