//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1289/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1289(t3431: f64, t6161: f64, t18499: f64, t18502: f64, t18508: f64, t18510: f64, t18518: f64, t18523: f64, t18525: f64, t18545: f64, t18553: f64, t18555: f64, t372: f64, t4261: f64, t4262: f64, t5544: f64) -> f64 {
    let t23915 = t3431 * t6161;
    let t23927 = 0.34299214494455789577e-2_f64 * t18499 + 0.17149607247227894789e-2_f64 * t18502 - 0.34299214494455789578e-1_f64 * t18508 - 0.40015750243531754508e-2_f64 * t18510 - 0.80031500487063509015e-2_f64 * t23915 + 0.34299214494455789578e-2_f64 * t18518 + 0.34299214494455789578e-2_f64 * t18523 - 0.51448821741683684367e-1_f64 * t18525 - 35.0_f64 / 54.0_f64 * t18545 - 0.34299214494455789578e-2_f64 * t18553 - 0.32012600194825403606e-1_f64 * t18555 - t4261 * t4262 * t5544 * t372 / 6.0_f64;
    t23927
}
