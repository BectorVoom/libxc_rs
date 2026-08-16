//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1262/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1262(t1892: f64, t785: f64, t1358: f64, t2439: f64, t1903: f64, t4075: f64, t1444: f64, t556: f64, t2782: f64, t212: f64, t5710: f64, t689: f64) -> (f64, f64, f64, f64) {
    let t13725 = t785 * t1892;
    let t13726 = t13725 * t1358;
    let t13727 = t2439 * t13726;
    let t13729 = t4075 * t1903;
    let t13730 = t13729 * t1444;
    let t13731 = t556 * t13730;
    let t13733 = 0.21951497276451705328e-1_f64 * t2782 * t13731;
    let t13734 = t212 * t5710;
    let t13735 = t13734 * t1358;
    let t13737 = 0.10975748638225852664e-1_f64 * t689 * t13735;
    (t13727, t13730, t13733, t13737)
}
