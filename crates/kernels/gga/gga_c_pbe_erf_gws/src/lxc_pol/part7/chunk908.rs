//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 908/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk908<F: Float>(t5211: F, t5523: F, t617: F, t7758: F, t17090: F, t17094: F, t17098: F, t17101: F, t17103: F, t17106: F, t17110: F, t17114: F, t17117: F, t17120: F) -> (F, F) {
    let t17124 = F::new(32.0) / F::new(9.0) * t5211 * t7758 * t617 * t5523;
    let t17125 = t17090 - t17094 + t17098 + t17101 + t17103 + t17106 - t17110 - t17114 - t17117 - t17120 - t17124;
    (t17124, t17125)
}
