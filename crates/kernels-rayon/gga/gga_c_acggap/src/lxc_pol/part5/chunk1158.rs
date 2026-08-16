//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1158/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1158(t5966: f64, t997: f64, t12855: f64, t12862: f64, t15851: f64, t15853: f64, t15855: f64, t15871: f64, t15891: f64, t15902: f64, t20875: f64, t20882: f64, t20888: f64, t20890: f64) -> f64 {
    let t20892 = t997 * t5966;
    let t20894 = 0.15117061203111996147e0_f64 * t15851 + 0.30234122406223992295e0_f64 * t15853 - 0.17149607247227894789e-2_f64 * t20875 + 0.34299214494455789578e-2_f64 * t15855 - 0.34299214494455789578e-2_f64 * t15871 - 0.40015750243531754508e-2_f64 * t12855 - 0.85748036236139473944e-3_f64 * t12862 - 0.25724410870841842183e-2_f64 * t15891 + 0.32012600194825403606e-1_f64 * t20882 + 0.7558530601555998074e-1_f64 * t15902 + 0.85748036236139473944e-2_f64 * t20888 + 0.40015750243531754508e-1_f64 * t20890 + 0.32012600194825403606e-1_f64 * t20892;
    t20894
}
