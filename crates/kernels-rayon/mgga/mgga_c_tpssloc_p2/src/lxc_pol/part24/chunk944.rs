//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 944/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk944(t2840: f64, t287: f64, t275: f64, t2793: f64, t912: f64, t2844: f64, t10294: f64, t10544: f64, t10296: f64, t10298: f64, t10300: f64, t10302: f64, t10307: f64, t10314: f64, t10320: f64, t10323: f64, t10530: f64, t10538: f64, t10547: f64, t10550: f64) -> (f64, f64, f64) {
    let t10660 = 1.0_f64 / t2840 / t287;
    let t10661 = t275 * t10660;
    let t10662 = t2793 * t912;
    let t10663 = t10662 * t2844;
    let t10665 = 0.96491876992155210402e2_f64 * t10661 * t10663;
    let t10675 = 0.36514074074074074075e0_f64 * t10294;
    let t10676 = 0.93011851851851851854e0_f64 * t10544;
    let t10680 = -0.59793333333333333333e0_f64 * t10530 - 0.27385555555555555556e0_f64 * t10296 + 0.16431333333333333333e0_f64 * t10302 + 0.5477111111111111111e-1_f64 * t10298 - 0.36514074074074074075e-1_f64 * t10307 - 0.82156666666666666667e-1_f64 * t10323 + 0.17938e1_f64 * t10538 - 0.82156666666666666668e-1_f64 * t10314 + 0.49293999999999999999e0_f64 * t10320 - t10675 - t10676 - 0.28483875e1_f64 * t10547 + 0.46074375e0_f64 * t10550 - 0.32862666666666666666e0_f64 * t10300;
    (t10662, t10665, t10680)
}
