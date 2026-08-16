//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 608/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk608(t1413: f64, t1449: f64, t3308: f64, t3311: f64, t3337: f64, t3340: f64, t3356: f64, t42: f64, t430: f64, t453: f64, t972: f64) -> (f64, f64) {
    let t3359 = 0.165625e-1_f64 * t3308 * t42 - 0.6625e-1_f64 * t1413 * t3311 + 0.165625e-1_f64 * t430 * t3337 + 0.496875e-1_f64 * t1449 * t3340 - 0.165625e-1_f64 * t453 * t3356;
    let t3363 = t972 * t972;
    (t3359, t3363)
}
