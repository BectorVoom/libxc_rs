//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1207/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1207(t119982: f64, t127676: f64, t31837: f64, t32471: f64, t98848: f64, t126110: f64, t119877: f64, t119879: f64, t121879: f64, t121881: f64, t121886: f64, t126182: f64, t126185: f64, t1949: f64, t28340: f64, t8649: f64, t8650: f64) -> f64 {
    let t127684 = t119982 * t127676;
    let t127689 = t98848 * t31837 * t32471;
    let t127692 = t126110 * t31837 * t32471;
    let t127694 = t121879 - t121881 + 0.57119737665102352616e0_f64 * t8649 * t8650 * t28340 * t1949 + 0.42839803248826764462e-1_f64 * t127684 + t119877 + t119879 - t121886 + 0.225875734067843736e-2_f64 * t126182 - 0.69416347856895220197e-2_f64 * t126185 - 0.14279934416275588154e-1_f64 * t127689 + 0.25389723392137995738e-1_f64 * t127692;
    t127694
}
