//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1142/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1142(t23789: f64, t2476: f64, t7501: f64, t845: f64, t2471: f64, t2475: f64, t241: f64, t7620: f64, t847: f64, t2441: f64, t7606: f64, t1000: f64, t176: f64, t23549: f64, t23642: f64, t23775: f64, t23781: f64, t23783: f64, t23788: f64, t2544: f64, t275: f64, t364: f64, t7254: f64, t7304: f64, t914: f64, t999: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t23793 = 0.6233672123775310788e3_f64 * t845 * t7501 * t23789 * t2476;
    let t23800 = t2471 * t2471;
    let t23801 = 1.0_f64 / t23800;
    let t23803 = t2475 * t2475;
    let t23804 = 1.0_f64 / t23803;
    let t23807 = 0.91080982599109921211e5_f64 * t845 * t23801 * t23789 * t23804;
    let t23808 = t241 * t7620;
    let t23810 = 0.23392893589820816284e1_f64 * t23808 * t847;
    let t23815 = 0.23392893589820816284e1_f64 * t2441 * t7606;
    let t23816 = t176 * t23775 * t275 * sigma0 * t364 / 2.0_f64 + 32.0_f64 / 9.0_f64 * t23781 + 2.0_f64 / 3.0_f64 * t23783 + t23788 - t23793 - 56.0_f64 / 9.0_f64 * t999 * t914 * t7254 * t23549 - 16.0_f64 / 3.0_f64 * t7304 * t2544 - t23807 - t23810 - t999 * t914 * t1000 * t23642 - t23815;
    (t23793, t23801, t23804, t23807, t23810, t23815, t23816)
}
