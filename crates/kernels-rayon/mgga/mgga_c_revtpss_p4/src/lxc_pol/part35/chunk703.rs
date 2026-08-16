//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 703/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk703(t7301: f64, t8099: f64, t545: f64, t8085: f64, t2028: f64, t1904: f64, t2027: f64, t2103: f64, t213: f64, t561: f64, t7295: f64, t7495: f64, t7498: f64, t7511: f64, t7517: f64, t7519: f64, t7917: f64, t8086: f64, t8095: f64) -> (f64, f64, f64, f64) {
    let t8100 = t7301 * t8099;
    let t8103 = t545 * t8085;
    let t8104 = t2028 * t8103;
    let t8107 = -t7495 + t7498 + 0.65854491829355115987e0_f64 * t213 * t8086 * t561 - 0.65854491829355115987e0_f64 * t7511 * t1904 + t7517 - t7519 - 0.4336814094102599731e0_f64 * t7917 * t2103 + 0.8673628188205199462e0_f64 * t7295 * t8095 + 0.4336814094102599731e0_f64 * t7295 * t8100 - 0.4336814094102599731e0_f64 * t2027 * t8104;
    (t8100, t8103, t8104, t8107)
}
