//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 339/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk339(t1112: f64, t1114: f64, t242: f64, t127: f64, t359: f64, t461: f64, t460: f64, t357: f64, t458: f64, t339: f64, t454: f64) -> (f64, f64, f64, f64) {
    let t1115 = t1112 * t1114;
    let t1116 = t242 * t1115;
    let t1120 = t359 * t127 * t461;
    let t1122 = t460 * t1120 / 4608.0_f64;
    let t1123 = t458 * t357;
    let t1125 = t339 * t454 * t1123;
    (t1116, t1120, t1122, t1125)
}
