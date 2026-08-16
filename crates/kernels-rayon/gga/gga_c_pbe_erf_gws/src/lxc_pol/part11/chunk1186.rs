//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1186/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1186(t17552: f64, t48106: f64, t48107: f64, t48108: f64, t48109: f64, t48112: f64, t48113: f64, t48114: f64, t48115: f64, t48117: f64, t48119: f64, t26341: f64, t32759: f64, t48120: f64, t48122: f64, t48124: f64, t48127: f64, t48128: f64, t48130: f64, t48132: f64, t48133: f64, t48134: f64, t48136: f64) -> (f64, f64) {
    let t48671 = t17552 - t48106 + t48107 - t48108 - t48109 + t48112 - t48113 - t48114 - t48115 + t48117 + t48119;
    let t48674 = t48120 + t48122 + t48124 + t48127 - t48128 + 8.0_f64 * t32759 - 0.38474813732852776452e0_f64 * t26341 + t48130 + t48132 + t48133 + t48134 + t48136;
    (t48671, t48674)
}
