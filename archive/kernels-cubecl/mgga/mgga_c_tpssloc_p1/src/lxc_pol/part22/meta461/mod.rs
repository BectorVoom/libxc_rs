//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta461 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1838;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1839;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1840;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta461<F: Float>(t19871: F, t3805: F, t6394: F, t19956: F, t550: F, t6347: F, t5249: F, t1799: F, t3792: F, t6414: F, t5248: F, t1367: F, t20416: F, t820: F, t1363: F, t16317: F, t16394: F, t19853: F, t19879: F, t20450: F, t3803: F, t5246: F, t6396: F, t1824: F, t6387: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t20454, t20460, t20463, t20465, t20470, t20473) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1838::<F>(t19871, t3805, t6394, t19956, t550, t6347, t5249, t1799, t3792, t6414);
        let (t20475, t20479, t20484) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1839::<F>(t20473, t5248, t5249, t1367, t20416, t820, t1363, t16317, t16394, t19853, t19879, t20450, t20454, t20460, t20465, t20470, t3803, t5246, t6396);
        let t20489 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1840::<F>(t1824, t6387);
    (t20454, t20460, t20463, t20465, t20470, t20473, t20475, t20479, t20484, t20489)
}
