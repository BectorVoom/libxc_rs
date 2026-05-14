//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 833/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk833<F: Float>(t17108: F, t5214: F, t1733: F, t1816: F, t5211: F, t5212: F, t4897: F, t5213: F, t5145: F, t5523: F, t617: F, t7758: F, t17090: F, t17094: F, t17098: F, t17101: F, t17103: F, t17106: F) -> (F, F, F, F, F, F) {
    let t17110 = 64.0 / 15.0 * t17108 * t5214;
    let t17114 = 32.0 / 15.0 * t5211 * t5212 * t1733 * t1816;
    let t17117 = 32.0 / 15.0 * t5211 * t5213 * t4897;
    let t17120 = 32.0 / 15.0 * t5211 * t5213 * t5145;
    let t17124 = 32.0 / 9.0 * t5211 * t7758 * t617 * t5523;
    let t17125 = t17090 - t17094 + t17098 + t17101 + t17103 + t17106 - t17110 - t17114 - t17117 - t17120 - t17124;
    (t17110, t17114, t17117, t17120, t17124, t17125)
}
