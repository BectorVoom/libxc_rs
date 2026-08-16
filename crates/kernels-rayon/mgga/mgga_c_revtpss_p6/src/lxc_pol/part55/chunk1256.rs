//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1256/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1256(t125648: f64, t32275: f64, t32707: f64, t27888: f64, t32716: f64, t34236: f64, t689: f64, t121365: f64, t125833: f64, t121208: f64, t122451: f64, t122454: f64, t125807: f64, t125814: f64, t125819: f64, t125826: f64, t125831: f64) -> (f64, f64) {
    let t128786 = t125648 * t32275 * t32707;
    let t128788 = t32716 * t27888;
    let t128790 = t34236 * t689;
    let t128791 = t121365 * t128790;
    let t128795 = 0.13223814266738539448e-3_f64 * t125833;
    let t128796 = 0.225875734067843736e-2_f64 * t125807 + t122451 - t121208 + 0.14874931683620404328e-2_f64 * t125814 - t122454 + 0.112937867033921868e-2_f64 * t125819 + 0.25389723392137995738e-1_f64 * t128786 - 0.25702851531048074406e-1_f64 * t128788 - 0.76169170176413987216e-1_f64 * t128791 - 0.26773803678175077509e-3_f64 * t125826 - 0.74374658418102021639e-4_f64 * t125831 + t128795;
    (t128790, t128796)
}
