//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta422 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1704;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1705;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta422(t1385: f64, t5353: f64, t3887: f64, t16413: f64, t539: f64, t225: f64, t5217: f64, t1834: f64, t3752: f64, t1323: f64, t5318: f64, t16122: f64, t562: f64, t1842: f64, t3911: f64, t3888: f64, t12021: f64, t12033: f64, t1375: f64, t1386: f64, t1843: f64, t3758: f64, t3882: f64, t3889: f64, t5215: f64, t5326: f64, t5354: f64, t568: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16452, t16453, t16458, t16460, t16463, t16465, t16468) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1704(t1385, t5353, t3887, t16413, t539, t225, t5217, t1834, t3752, t1323, t5318, t16122, t562);
        let (t16470, t16471, t16474, t16475, t16485) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1705(t1842, t3911, t3887, t3888, t12021, t12033, t1375, t1386, t16453, t16458, t16460, t16463, t16465, t16468, t1843, t3758, t3882, t3889, t5215, t5326, t5354, t568);
    (t16452, t16453, t16458, t16460, t16463, t16465, t16468, t16470, t16471, t16474, t16475, t16485)
}
