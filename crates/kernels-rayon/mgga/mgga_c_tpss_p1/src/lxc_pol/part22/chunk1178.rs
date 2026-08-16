//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1178/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1178(t10445: f64, t10456: f64, t1165: f64, t13133: f64, t13136: f64, t13146: f64, t13220: f64, t1338: f64, t2056: f64, t2105: f64, t3493: f64, t3537: f64, t4347: f64, t645: f64, t7798: f64) -> f64 {
    let t13223 = 4.0_f64 * t10456 * t1338 + 2.0_f64 * t1165 * t13220 + 4.0_f64 * t13133 * t645 + 2.0_f64 * t13146 * t1338 + 2.0_f64 * t1338 * t7798 + 4.0_f64 * t2056 * t3537 + 2.0_f64 * t2105 * t3493 + 4.0_f64 * t3537 * t4347 + t10445 + 2.0_f64 * t13136;
    t13223
}
