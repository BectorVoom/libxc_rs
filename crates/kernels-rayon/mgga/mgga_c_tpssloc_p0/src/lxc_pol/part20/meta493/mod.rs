//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta493 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1990;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1991;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1992;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta493(t1362: f64, t16060: f64, t12339: f64, t1831: f64, t3866: f64, t5314: f64, t1367: f64, t16018: f64, t820: f64, t3865: f64, t5234: f64, t1369: f64, t12189: f64, t1811: f64, t1358: f64, t5231: f64, t16123: f64, t554: f64, t1815: f64, t3862: f64, t3726: f64, t5227: f64, t119: f64, t210: f64, t12308: f64, t12310: f64, t12317: f64, t12323: f64, t12325: f64, t12330: f64, t12336: f64, t1315: f64, t1363: f64, t3783: f64, t3876: f64, t5240: f64, t559: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t16321, t16325, t16331, t16333, t16336, t16338) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1990(t1362, t16060, t12339, t1831, t3866, t5314, t1367, t16018, t820, t3865, t5234, t1369);
        let (t16341, t16346, t16347, t16350, t16354, t16355) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1991(t12189, t1811, t1358, t5231, t16123, t554, t1815, t3862, t3726, t5227, t119, t16018);
        let (t16356, t16361) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1992(t16355, t210, t12308, t12310, t12317, t12323, t12325, t12330, t12336, t1315, t1363, t1369, t16321, t16325, t16331, t16333, t16338, t16341, t16346, t16347, t16350, t16354, t1831, t3783, t3876, t5240, t5314, t559);
    (t16321, t16333, t16336, t16347, t16355, t16356, t16361)
}
