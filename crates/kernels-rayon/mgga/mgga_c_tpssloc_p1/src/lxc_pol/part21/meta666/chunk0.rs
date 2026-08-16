//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2467/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2467(t11887: f64, t44690: f64, t42339: f64, t466: f64, t11715: f64, t42341: f64, t11721: f64, t23508: f64, t11714: f64, t476: f64, t3508: f64, t11883: f64, t3493: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t44691 = t44690 * t11887;
    let t44696 = t466 * t42339;
    let t44698 = t44696 * t42341 * t11715;
    let t44701 = t23508 * t11721;
    let t44722 = 1.0_f64 / t11714 / t476;
    let t44724 = t44696 * t42341 * t44722;
    let t44725 = t3508 * t3508;
    let t44726 = t23508 * t44725;
    let t44730 = t11883 * t3493;
    (t44691, t44696, t44698, t44701, t44722, t44724, t44725, t44726, t44730)
}
