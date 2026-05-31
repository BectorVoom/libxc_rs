//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1142/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1142<F: Float>(t23789: F, t2476: F, t7501: F, t845: F, t2471: F, t2475: F, t241: F, t7620: F, t847: F, t2441: F, t7606: F, t1000: F, t176: F, t23549: F, t23642: F, t23775: F, t23781: F, t23783: F, t23788: F, t2544: F, t275: F, t364: F, t7254: F, t7304: F, t914: F, t999: F, sigma0: F) -> (F, F, F, F, F, F, F) {
    let t23793 = F::cast_from(0.6233672123775310788e3_f64) * t845 * t7501 * t23789 * t2476;
    let t23800 = t2471 * t2471;
    let t23801 = F::cast_from(1.0_f64) / t23800;
    let t23803 = t2475 * t2475;
    let t23804 = F::cast_from(1.0_f64) / t23803;
    let t23807 = F::cast_from(0.91080982599109921211e5_f64) * t845 * t23801 * t23789 * t23804;
    let t23808 = t241 * t7620;
    let t23810 = F::cast_from(0.23392893589820816284e1_f64) * t23808 * t847;
    let t23815 = F::cast_from(0.23392893589820816284e1_f64) * t2441 * t7606;
    let t23816 = t176 * t23775 * t275 * sigma0 * t364 / F::cast_from(2.0_f64) + F::cast_from(32.0_f64) / F::cast_from(9.0_f64) * t23781 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t23783 + t23788 - t23793 - F::cast_from(56.0_f64) / F::cast_from(9.0_f64) * t999 * t914 * t7254 * t23549 - F::cast_from(16.0_f64) / F::cast_from(3.0_f64) * t7304 * t2544 - t23807 - t23810 - t999 * t914 * t1000 * t23642 - t23815;
    (t23793, t23801, t23804, t23807, t23810, t23815, t23816)
}
