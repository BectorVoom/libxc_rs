//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2212/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2212(t23499: f64, t2908: f64, t141: f64, t23503: f64, t930: f64, t15123: f64, t15189: f64, t23472: f64, t23476: f64, t23479: f64, t23483: f64, t23487: f64, t23490: f64, t23493: f64, t23496: f64, t23501: f64, t23505: f64) -> (f64, f64, f64, f64, f64) {
    let t23507 = t2908 * t23499;
    let t23508 = t141 * t23507;
    let t23510 = t930 * t23503;
    let t23511 = t141 * t23510;
    let t23514 = -0.36514074074074074075e-1_f64 * t23472 - 0.82156666666666666667e-1_f64 * t23476 - 0.33218518518518518518e0_f64 * t23479 + 0.11958666666666666667e1_f64 * t23483 - 0.17938e1_f64 * t23487 - 0.29896666666666666667e0_f64 * t23490 + 0.16431333333333333333e0_f64 * t23493 - 0.49293999999999999999e0_f64 * t23496 - 0.27385555555555555556e0_f64 * t15123 - 0.59793333333333333333e0_f64 * t23501 + 0.17938e1_f64 * t23505 - 0.82156666666666666668e-1_f64 * t23508 + 0.49293999999999999999e0_f64 * t23511 - 0.39862222222222222223e0_f64 * t15189;
    (t23507, t23508, t23510, t23511, t23514)
}
