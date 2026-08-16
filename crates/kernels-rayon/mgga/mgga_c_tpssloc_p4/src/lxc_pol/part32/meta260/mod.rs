//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta260 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1170;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1171;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1172;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1173;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta260(t1351: f64, t562: f64, t550: f64, t6976: f64, t1992: f64, t1372: f64, t1998: f64, t214: f64, t1985: f64, t1338: f64, t2006: f64, t1352: f64, t553: f64, t6955: f64, t1332: f64, t1336: f64, t2013: f64, t544: f64, t6967: f64, t6971: f64, t6975: f64, t1378: f64, t1375: f64, t1386: f64, t2016: f64, t3758: f64, t3882: f64, t568: f64, t6885: f64, t6893: f64, t6900: f64, t6904: f64, t6909: f64, t6911: f64, t6956: f64, t6958: f64, t6963: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6978, t6979, t6980, t6982, t6983, t6984, t6987) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1170(t1351, t562, t550, t6976, t1992, t1372, t1998, t214, t1985, t1338, t2006);
        let (t6988, t6990, t6992) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1171(t1352, t6987, t553, t6955, t1332, t1336, t2013, t544, t6967, t6971, t6975, t6980, t6984);
        let t6993 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1172(t1378, t6992);
        let t6995 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1173(t1375, t1386, t2016, t3758, t3882, t568, t6885, t6893, t6900, t6904, t6909, t6911, t6956, t6958, t6963, t6993);
    (t6978, t6979, t6982, t6983, t6987, t6988, t6990, t6992, t6993, t6995)
}
