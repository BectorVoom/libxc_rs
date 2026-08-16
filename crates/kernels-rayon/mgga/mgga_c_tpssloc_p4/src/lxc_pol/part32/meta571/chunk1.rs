//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1946/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1946(t28: f64, t5527: f64, t1915: f64, t23788: f64, t28248: f64, t1484: f64, t1649: f64, t5544: f64, t5664: f64, t1530: f64, t5660: f64, t1877: f64, t22959: f64, t23295: f64, t2522: f64, t25358: f64, t28448: f64, t4314: f64, t5966: f64, t6670: f64, t7541: f64, t7649: f64, t7656: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t28764 = t28 * t5527;
    let t28765 = t1915 * t28764;
    let t28771 = t23788 * t28248;
    let t28774 = t1649 * t1484;
    let t28778 = t28 * t5544;
    let t28789 = t28 * t5664;
    let t28792 = t1649 * t1530;
    let t28795 = t28 * t5660;
    let t28802 = 3.0_f64 * t4314 * t28765 + 3.0_f64 * t2522 * t7541 * t7649 - 3.0_f64 * t22959 * t28771 + 3.0_f64 * t2522 * t1915 * t28774 + 3.0_f64 / 2.0_f64 * t2522 * t1915 * t28778 + t1877 * t28448 * t28 / 2.0_f64 - t1877 * t25358 * t7656 + t1877 * t7541 * t1649 + t1877 * t23295 * t28789 - t1877 * t6670 * t28792 - t1877 * t6670 * t28795 / 2.0_f64 + t1877 * t1915 * t5966 / 2.0_f64;
    (t28764, t28765, t28771, t28774, t28778, t28789, t28792, t28795, t28802)
}
