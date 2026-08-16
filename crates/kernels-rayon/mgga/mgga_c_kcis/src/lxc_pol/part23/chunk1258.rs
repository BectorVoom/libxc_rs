//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1258/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1258(t16974: f64, t2237: f64, t27369: f64, t27372: f64, t28369: f64, t28535: f64, t6176: f64, t7895: f64, t7914: f64, t94539: f64, t94546: f64, t94554: f64, t98119: f64, t98445: f64, t98538: f64, t98543: f64, t98553: f64) -> f64 {
    let t98561 = -0.73697530864197530861e-3_f64 * t94539 + 0.61836467013888888888e-4_f64 * t98538 - 0.46336805555555555556e-3_f64 * t94546 + 0.23168402777777777778e-3_f64 * t94554 - 0.55273148148148148147e-3_f64 * t98543 + 0.13901041666666666667e-2_f64 * t7895 * t28535 + 0.69505208333333333333e-3_f64 * t2237 * t6176 * t7914 * t16974 - 0.16581944444444444444e-2_f64 * t98553 - 0.92754700520833333333e-4_f64 * t27369 * t98445 - 0.13901041666666666667e-2_f64 * t28369 * t27372 - 0.18550940104166666667e-3_f64 * t98119 * t27372;
    t98561
}
