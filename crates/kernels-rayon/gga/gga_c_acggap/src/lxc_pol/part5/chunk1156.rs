//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1156/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1156(t3431: f64, t5623: f64, t12838: f64, t12840: f64, t12842: f64, t12844: f64, t12848: f64, t15826: f64, t15828: f64, t15830: f64, t15832: f64, t15841: f64, t15849: f64) -> f64 {
    let t20857 = t3431 * t5623;
    let t20870 = 0.32012600194825403606e-1_f64 * t20857 - 0.34299214494455789578e-2_f64 * t15826 - 0.24009450146119052704e-1_f64 * t15828 - 0.16006300097412701803e-1_f64 * t15830 - 0.12004725073059526352e-1_f64 * t15832 + 0.34299214494455789578e-2_f64 * t12838 - 0.25724410870841842183e-2_f64 * t12840 - 0.17149607247227894789e-2_f64 * t12842 + 0.25724410870841842183e-2_f64 * t12844 - 0.80031500487063509016e-2_f64 * t12848 + 0.17149607247227894789e-1_f64 * t15841 - 0.68026775414003982663e-1_f64 * t15849;
    t20870
}
