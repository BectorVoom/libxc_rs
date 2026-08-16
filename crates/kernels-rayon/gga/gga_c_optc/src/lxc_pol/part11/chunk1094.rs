//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1094/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1094(t2517: f64, t4863: f64, t4919: f64, t7504: f64, t2492: f64, t2619: f64, t4963: f64, t874: f64, t4933: f64, t530: f64, t862: f64, t4961: f64, t7467: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41291 = t4863 * t2517;
    let t41392 = t4919 * t7504;
    let t41396 = t4863 * t2492;
    let t41484 = t874 * t2619 * t4963;
    let t41498 = t862 * t530 * t4933;
    let t41521 = t7467 * t4961;
    (t41291, t41392, t41396, t41484, t41498, t41521)
}
