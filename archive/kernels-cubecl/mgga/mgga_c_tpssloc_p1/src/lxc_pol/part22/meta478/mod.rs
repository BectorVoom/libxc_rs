//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta478 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1876;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1877;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1878;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta478<F: Float>(t20861: F, t819: F, t820: F, t20853: F, t232: F, t5527: F, t4181: F, t9646: F, t16839: F, t2645: F, t5591: F, t1484: F, t2632: F, t5611: F, t4180: F, t119: F, t20800: F, t210: F, t13251: F, t16940: F, t2630: F, t2643: F, t4167: F, t4178: F, t5593: F, t5614: F, t5619: F, t787: F, t817: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t20963, t20969, t20972, t20974, t20978, t20981) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1876::<F>(t20861, t819, t820, t20853, t232, t5527, t4181, t9646, t16839, t2645, t5591, t1484, t2632);
        let (t20983, t20986) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1877::<F>(t16839, t20981, t2645, t2632, t5611);
        let (t20988, t20993, t20994, t20998) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1878::<F>(t20986, t4180, t4181, t119, t20800, t210, t13251, t16940, t20963, t20969, t20974, t20978, t20983, t2630, t2643, t4167, t4178, t5593, t5614, t5619, t787, t817);
    (t20963, t20969, t20972, t20974, t20978, t20983, t20986, t20988, t20993, t20994, t20998)
}
