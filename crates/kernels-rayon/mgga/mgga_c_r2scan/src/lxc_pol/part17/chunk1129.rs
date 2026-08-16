//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1129/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1129(t2381: f64, t37028: f64, t37078: f64, t1010: f64, t11056: f64, t1276: f64, t2391: f64, t3366: f64, t11050: f64, t8358: f64, t11885: f64, t6654: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40781 = t37028 * t2381;
    let t40786 = 44.0_f64 / 9.0_f64 * t37078;
    let t40788 = t1276 * t11056 * t1010;
    let t40797 = t1276 * t3366 * t2391;
    let t40804 = t8358 * t11050;
    let t40806 = t6654 * t11885;
    (t40781, t40786, t40788, t40797, t40804, t40806)
}
