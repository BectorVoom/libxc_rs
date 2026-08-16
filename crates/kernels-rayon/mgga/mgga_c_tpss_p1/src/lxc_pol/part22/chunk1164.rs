//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1164/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1164(t3342: f64, t4484: f64, t1248: f64, t12810: f64, t774: f64, t1646: f64, t9994: f64, t10137: f64, t4405: f64, t1206: f64, t4408: f64, t762: f64) -> (f64, f64, f64, f64, f64) {
    let t13013 = 7.0_f64 / 576.0_f64 * t3342 * t4484;
    let t13015 = t1248 * t774 * t12810;
    let t13018 = t9994 * t1646;
    let t13021 = 7.0_f64 / 24.0_f64 * t10137 * t4405;
    let t13023 = t762 * t4408 * t1206;
    (t13013, t13015, t13018, t13021, t13023)
}
