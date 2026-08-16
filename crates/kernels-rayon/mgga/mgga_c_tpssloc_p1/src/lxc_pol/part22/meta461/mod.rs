//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta461 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1838;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1839;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1840;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta461(t19871: f64, t3805: f64, t6394: f64, t19956: f64, t550: f64, t6347: f64, t5249: f64, t1799: f64, t3792: f64, t6414: f64, t5248: f64, t1367: f64, t20416: f64, t820: f64, t1363: f64, t16317: f64, t16394: f64, t19853: f64, t19879: f64, t20450: f64, t3803: f64, t5246: f64, t6396: f64, t1824: f64, t6387: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t20454, t20460, t20463, t20465, t20470, t20473) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1838(t19871, t3805, t6394, t19956, t550, t6347, t5249, t1799, t3792, t6414);
        let (t20475, t20479, t20484) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1839(t20473, t5248, t5249, t1367, t20416, t820, t1363, t16317, t16394, t19853, t19879, t20450, t20454, t20460, t20465, t20470, t3803, t5246, t6396);
        let t20489 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1840(t1824, t6387);
    (t20454, t20460, t20463, t20465, t20470, t20473, t20475, t20479, t20484, t20489)
}
