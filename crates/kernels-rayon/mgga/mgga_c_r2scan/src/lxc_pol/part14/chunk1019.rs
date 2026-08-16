//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1019/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1019(t322: f64, t1018: f64, t3517: f64, t1125: f64, t2405: f64, t12240: f64) -> (f64, f64, f64) {
    let t332 = 0.25e1_f64 < t322;
    let t12271 = t3517 * t1018;
    let t12273 = t1125 * t2405;
    let t12285 = piecewise3(t332, 0.0_f64, t12240);
    (t12271, t12273, t12285)
}
