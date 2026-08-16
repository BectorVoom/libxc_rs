//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 695/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk695(t234: f64, t5300: f64, t1814: f64, t732: f64, t1813: f64, t1841: f64, t148: f64, t1683: f64, t5245: f64, t22: f64, t502: f64, t1712: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5302 = 0.6233709278045326953e3_f64 * t234 * t5300;
    let t5303 = t732 * t1814;
    let t5305 = t1841 * t1813;
    let t5307 = 0.51947577317044391277e2_f64 * t234 * t5305;
    let t5308 = t148 * t1683;
    let t5309 = t5308 * t5245;
    let t5311 = t22 * t502;
    let t5312 = t1712 * t5311;
    (t5302, t5303, t5307, t5309, t5311, t5312)
}
