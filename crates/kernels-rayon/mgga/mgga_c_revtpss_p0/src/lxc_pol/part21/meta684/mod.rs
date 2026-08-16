//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta684 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2499;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2500;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta684(t371: f64, t481: f64, t482: f64, t9291: f64, t12627: f64, t1284: f64, t3624: f64, t12910: f64, t12911: f64, t12916: f64, t12640: f64, t127: f64, t12866: f64, t3630: f64, t3712: f64, t12809: f64, t12811: f64, t12952: f64, t3172: f64, t3711: f64, t12901: f64, t13033: f64, t13042: f64, t13047: f64, t3555: f64, t3781: f64, t5330: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t44607, t44609, t44616, t44624, t44634) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2499(t371, t481, t482, t9291, t12627, t1284, t3624, t12910, t12911, t12916, t12640, t127, t12866, t3630, t3712);
        let (t44637, t44649, t44658, t44661, t44664) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2500(t12809, t12811, t12916, t12952, t3172, t3711, t12901, t13033, t13042, t13047, t3555, t3781, t5330);
    (t44607, t44609, t44616, t44624, t44634, t44637, t44649, t44658, t44661, t44664)
}
