//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 582/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk582(t1736: f64, t630: f64, t1744: f64, t1746: f64, t4834: f64, t4887: f64, t4838: f64, t4842: f64, t4845: f64, t4848: f64, t4866: f64, t4874: f64, t4882: f64, t4884: f64, t4891: f64, t4895: f64, t4898: f64, t4901: f64) -> (f64, f64, f64, f64) {
    let t4927 = t1736 * t630;
    let t4928 = 1.0_f64 / t4927;
    let t4929 = t1744 * t1744;
    let t4931 = t4928 * t4929 * t1746;
    let t4936 = 0.40256666666666666667e0_f64 * t4834;
    let t4943 = 0.27595e0_f64 * t4887;
    let t4948 = -0.1294625e1_f64 * t4866 + 0.258925e1_f64 * t4874 + t4936 + 0.20128333333333333334e0_f64 * t4838 - 0.20128333333333333333e0_f64 * t4842 + 0.60385e0_f64 * t4845 - 0.301925e0_f64 * t4848 + 0.82524375e-1_f64 * t4882 + 0.16504875e0_f64 * t4884 + t4943 + 0.22076e0_f64 * t4891 - 0.5519e-1_f64 * t4895 + 0.33114e0_f64 * t4898 - 0.16557e0_f64 * t4901;
    (t4928, t4929, t4931, t4948)
}
