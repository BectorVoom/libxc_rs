//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta496 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2000;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2001;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta496(t16131: f64, t16435: f64, t1378: f64, t225: f64, t5319: f64, t1372: f64, t5210: f64, t12030: f64, t12444: f64, t1375: f64, t1386: f64, t16022: f64, t16028: f64, t16030: f64, t1843: f64, t3758: f64, t3889: f64, t3912: f64, t5215: f64, t5321: f64, t5354: f64, t568: f64, t1385: f64, t5353: f64, t3887: f64, t16413: f64, t539: f64, t5217: f64, t1834: f64, t3752: f64, t1323: f64, t5318: f64, t16122: f64, t562: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16436, t16437, t16439, t16448, t16451) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2000(t16131, t16435, t1378, t225, t5319, t1372, t5210, t12030, t12444, t1375, t1386, t16022, t16028, t16030, t1843, t3758, t3889, t3912, t5215, t5321, t5354, t568);
        let (t16453, t16458, t16460, t16463, t16465, t16468) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2001(t1385, t5353, t3887, t16413, t539, t225, t5217, t1834, t3752, t1323, t5318, t16122, t562);
    (t16436, t16437, t16439, t16448, t16451, t16453, t16458, t16460, t16463, t16465, t16468)
}
