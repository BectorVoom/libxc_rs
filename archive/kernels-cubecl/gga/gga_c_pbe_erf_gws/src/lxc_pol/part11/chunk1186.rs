//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1186/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1186<F: Float>(t17552: F, t48106: F, t48107: F, t48108: F, t48109: F, t48112: F, t48113: F, t48114: F, t48115: F, t48117: F, t48119: F, t26341: F, t32759: F, t48120: F, t48122: F, t48124: F, t48127: F, t48128: F, t48130: F, t48132: F, t48133: F, t48134: F, t48136: F) -> (F, F) {
    let t48671 = t17552 - t48106 + t48107 - t48108 - t48109 + t48112 - t48113 - t48114 - t48115 + t48117 + t48119;
    let t48674 = t48120 + t48122 + t48124 + t48127 - t48128 + F::cast_from(8.0_f64) * t32759 - F::cast_from(0.38474813732852776452e0_f64) * t26341 + t48130 + t48132 + t48133 + t48134 + t48136;
    (t48671, t48674)
}
