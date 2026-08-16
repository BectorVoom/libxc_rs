//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1233/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1233(t1299: f64, t3633: f64, t11056: f64, t2378: f64, t2381: f64, t37028: f64, t37078: f64, t1010: f64, t1276: f64, t11053: f64, t8358: f64, t19141: f64, t3629: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t40770 = t3633 * t1299;
    let t40779 = t2378 * t11056;
    let t40781 = t37028 * t2381;
    let t40782 = 4.0_f64 / 3.0_f64 * t40781;
    let t40786 = 44.0_f64 / 9.0_f64 * t37078;
    let t40788 = t1276 * t11056 * t1010;
    let t40790 = t8358 * t11053;
    let t40792 = t19141 * t3629;
    (t40770, t40779, t40782, t40786, t40788, t40790, t40792)
}
