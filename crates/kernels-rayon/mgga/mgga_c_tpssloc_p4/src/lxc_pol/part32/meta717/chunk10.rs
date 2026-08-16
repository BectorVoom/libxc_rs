//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2281/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2281(t1649: f64, t4119: f64, t23788: f64, t67123: f64, t1081: f64, t5660: f64, t5544: f64, t16662: f64, t28: f64, t5527: f64, t1877: f64, t1915: f64, t22959: f64, t2522: f64, t25901: f64, t25905: f64, t25928: f64, t25938: f64, t28448: f64, t28764: f64, t28765: f64, t4314: f64, t46341: f64, t5966: f64, t6666: f64, t6670: f64, t6841: f64, t7541: f64, t98027: f64) -> f64 {
    let t100718 = t1649 * t4119;
    let t100731 = t23788 * t67123;
    let t100734 = t1081 * t5660;
    let t100743 = t1081 * t5544;
    let t100747 = t28 * t16662;
    let t100759 = t1081 * t5527;
    let t100763 = 3.0_f64 * t2522 * t1915 * t100718 + 3.0_f64 * t2522 * t7541 * t25901 + 3.0_f64 * t4314 * t6666 * t28764 + 3.0_f64 * t2522 * t7541 * t25905 - 3.0_f64 / 2.0_f64 * t22959 * t100731 - t1877 * t6670 * t100734 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t2522 * t28448 * t6841 + 2.0_f64 * t98027 * t25928 + 3.0_f64 / 2.0_f64 * t2522 * t1915 * t100743 + 3.0_f64 / 2.0_f64 * t2522 * t1915 * t100747 + 3.0_f64 * t46341 * t28765 + t1877 * t6666 * t5966 / 2.0_f64 + 3.0_f64 * t2522 * t7541 * t25938 + 3.0_f64 * t4314 * t1915 * t100759;
    t100763
}
