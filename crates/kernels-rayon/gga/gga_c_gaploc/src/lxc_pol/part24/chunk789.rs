//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 789/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk789(t2089: f64, t911: f64, t7419: f64, t7427: f64, t2604: f64, t6135: f64, t1835: f64, t733: f64, t2365: f64, t2022: f64, t701: f64, t7291: f64) -> (f64, f64, f64, f64) {
    let t7428 = t911 * t2089;
    let t7429 = t7428 * t7419;
    let t7430 = t7427 * t7429;
    let t7432 = t6135 * t2604;
    let t7434 = t733 * t1835;
    let t7435 = t2365 * t7434;
    let t7436 = t2022 * t7435;
    let t7438 = t7291 * t701;
    (t7430, t7432, t7436, t7438)
}
