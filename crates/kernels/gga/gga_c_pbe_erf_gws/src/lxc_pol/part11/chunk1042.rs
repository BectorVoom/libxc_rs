//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1042/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1042<F: Float>(t17548: F, t26328: F, t48090: F, t48092: F, t48095: F, t48099: F, t48101: F, t48102: F, t48103: F, t48104: F, t48105: F, t17552: F, t48106: F, t48107: F, t48108: F, t48109: F, t48112: F, t48113: F, t48114: F, t48115: F, t48117: F, t48119: F) -> (F, F) {
    let t48669 = -t48090 + t48092 - t48095 - t48099 - t48101 + 16.0 / 3.0 * t26328 - t48102 + t48103 - t48104 - t48105 + t17548;
    let t48671 = t17552 - t48106 + t48107 - t48108 - t48109 + t48112 - t48113 - t48114 - t48115 + t48117 + t48119;
    (t48669, t48671)
}
