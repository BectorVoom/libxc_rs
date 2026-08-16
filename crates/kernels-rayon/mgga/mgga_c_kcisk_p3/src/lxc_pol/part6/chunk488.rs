//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 488/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk488(t4856: f64, t608: f64, t606: f64, t609: f64, t4834: f64, t353: f64, t579: f64, t964: f64, t163: f64, t657: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4857 = 1.0_f64 / t4856;
    let t4858 = t608 * t4857;
    let t4864 = 1.0_f64 / t609 / t606;
    let t4868 = 4.0_f64 / 9.0_f64 * t4834;
    let t4876 = 0.39862222222222222223e0_f64 * t4834;
    let t4881 = 1.0_f64/f64::sqrt(t606);
    let t4887 = t353 * t964 * t579;
    let t4888 = 0.27385555555555555555e0_f64 * t4887;
    let t4889 = t163 * t657;
    (t4857, t4858, t4864, t4868, t4876, t4881, t4887, t4888, t4889)
}
