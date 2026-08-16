//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1459/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1459<F: Float>(t43994: F, t43997: F, t44000: F, t44002: F, t44006: F, t44072: F, t44080: F, t44082: F, t44085: F, t44089: F, t44092: F, t44369: F) -> F {
    let t44795 = t43994 - t43997 - t44000 + t44002 + t44006 + t44072 + t44080 + t44082 - t44085 - t44089 + t44092 + t44369;
    t44795
}
