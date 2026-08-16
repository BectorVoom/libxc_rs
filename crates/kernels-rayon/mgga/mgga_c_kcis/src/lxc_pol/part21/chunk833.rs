//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 833/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk833(t1094: f64, t3423: f64, t284: f64, t3473: f64, t3177: f64, t3436: f64, t1194: f64, t381: f64, t1095: f64, t1169: f64, t983: f64, t9538: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10691 = t3423 * t1094;
    let t10692 = t10691 * sigma0;
    let t10707 = t3473 * t284;
    let t10745 = t3177 * t3436;
    let t10752 = t381 * t1194;
    let t10753 = t1095 * t10752;
    let t10787 = t1169 * t983;
    let t10799 = t9538 * t381;
    (t10691, t10692, t10707, t10745, t10752, t10753, t10787, t10799)
}
