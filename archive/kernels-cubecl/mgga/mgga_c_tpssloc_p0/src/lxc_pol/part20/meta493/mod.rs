//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta493 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1990;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1991;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1992;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta493<F: Float>(t1362: F, t16060: F, t12339: F, t1831: F, t3866: F, t5314: F, t1367: F, t16018: F, t820: F, t3865: F, t5234: F, t1369: F, t12189: F, t1811: F, t1358: F, t5231: F, t16123: F, t554: F, t1815: F, t3862: F, t3726: F, t5227: F, t119: F, t210: F, t12308: F, t12310: F, t12317: F, t12323: F, t12325: F, t12330: F, t12336: F, t1315: F, t1363: F, t3783: F, t3876: F, t5240: F, t559: F) -> (F, F, F, F, F, F, F) {
        let (t16321, t16325, t16331, t16333, t16336, t16338) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1990::<F>(t1362, t16060, t12339, t1831, t3866, t5314, t1367, t16018, t820, t3865, t5234, t1369);
        let (t16341, t16346, t16347, t16350, t16354, t16355) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1991::<F>(t12189, t1811, t1358, t5231, t16123, t554, t1815, t3862, t3726, t5227, t119, t16018);
        let (t16356, t16361) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1992::<F>(t16355, t210, t12308, t12310, t12317, t12323, t12325, t12330, t12336, t1315, t1363, t1369, t16321, t16325, t16331, t16333, t16338, t16341, t16346, t16347, t16350, t16354, t1831, t3783, t3876, t5240, t5314, t559);
    (t16321, t16333, t16336, t16347, t16355, t16356, t16361)
}
