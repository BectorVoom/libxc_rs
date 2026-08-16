//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2325/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2325(t25927: f64, t98111: f64, t100682: f64, t100689: f64, t100692: f64, t100696: f64, t100705: f64, t100708: f64, t18196: f64, t1877: f64, t1915: f64, t22959: f64, t25013: f64, t2522: f64, t25358: f64, t25372: f64, t25898: f64, t25945: f64, t28: f64, t28778: f64, t28789: f64, t6666: f64, t6670: f64, t6848: f64, t81539: f64, t86736: f64, t98054: f64, t98071: f64, t99043: f64) -> f64 {
    let t100713 = t25927 * t98111;
    let t100716 = -t1877 * t25358 * t25945 + t1877 * t81539 * t28789 + t1877 * t1915 * t18196 / 2.0_f64 - 3.0_f64 * t25372 * t100682 - t1877 * t98054 * t6848 / 2.0_f64 + 2.0_f64 * t25372 * t100689 + 3.0_f64 / 2.0_f64 * t2522 * t1915 * t100692 - t1877 * t6670 * t100696 + 3.0_f64 / 2.0_f64 * t2522 * t6666 * t28778 + t1877 * t99043 * t28 / 2.0_f64 - 3.0_f64 * t22959 * t100705 + 6.0_f64 * t25013 * t100708 - 3.0_f64 * t86736 * t25898 + 6.0_f64 * t22959 * t100713 + t98071;
    t100716
}
