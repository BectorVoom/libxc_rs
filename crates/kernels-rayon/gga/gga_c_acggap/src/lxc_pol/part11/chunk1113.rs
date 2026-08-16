//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1113/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1113(t1967: f64, t8978: f64, t31095: f64, t31100: f64, t31029: f64, t31033: f64, t31037: f64, t31039: f64, t31041: f64, t31045: f64, t31049: f64, t31060: f64, t31074: f64, t31081: f64, t31083: f64, t35259: f64, t35261: f64, t35264: f64, t35271: f64) -> f64 {
    let t35273 = t1967 * t8978;
    let t35274 = 0.25724410870841842184e-2_f64 * t35273;
    let t35278 = 0.17149607247227894789e-2_f64 * t31095;
    let t35279 = 0.42874018118069736972e-2_f64 * t31100;
    let t35280 = 0.22921875e-1_f64 * t31029 + 0.4584375e-1_f64 * t31033 + t31037 + 0.80031500487063509016e-2_f64 * t31039 - 0.42874018118069736972e-3_f64 * t31041 + t35259 - t35261 + 0.31448092289604152068e-3_f64 * t35264 - 0.32155513588552302729e-2_f64 * t31045 + 0.14291339372689912324e-3_f64 * t31049 + 0.94344276868812456204e-3_f64 * t31060 - 0.10718504529517434243e-3_f64 * t35271 + t35274 + 0.17149607247227894789e-2_f64 * t31074 - 0.420234375e-1_f64 * t31081 - 0.28015625e-1_f64 * t31083 - t35278 - t35279;
    t35280
}
