//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 767/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk767(t1031: f64, t3491: f64, t184: f64, t221: f64, t3390: f64, t7027: f64, t1621: f64, t1620: f64, t2612: f64, t3500: f64, t12339: f64, t5008: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12527 = t3491 * t1031;
    let t12528 = t12527 * t184;
    let t12530 = 4.0_f64 / 5.0_f64 * t12528 * t221;
    let t12531 = t7027 * t3390;
    let t12532 = t1621 * t12531;
    let t12534 = 8.0_f64 / 5.0_f64 * t1620 * t12532;
    let t12536 = 8.0_f64 / 15.0_f64 * t2612 * t3500;
    let t12537 = t5008 * t12339;
    (t12527, t12528, t12530, t12531, t12532, t12534, t12536, t12537)
}
