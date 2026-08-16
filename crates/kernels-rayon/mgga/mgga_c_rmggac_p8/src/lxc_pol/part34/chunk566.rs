//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 566/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk566(t14567: f64, t289: f64, t13970: f64, t13976: f64, t2012: f64, t2265: f64, t2010: f64, t3194: f64, t4965: f64, t1356: f64, t14498: f64, t2144: f64, t698: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14568 = t289 * t14567;
    let t14569 = 0.2363e1_f64 * t14568;
    let t14570 = 0.68186654135613354325e-2_f64 * t13970;
    let t14571 = 0.85129199786595678799e-5_f64 * t13976;
    let t14572 = t2012 * t2265;
    let t14573 = t2010 * t14572;
    let t14574 = 0.36021158228745895953e-3_f64 * t14573;
    let t14576 = t4965 * t3194;
    let t14577 = 0.39914139006212695214e-1_f64 * t14576;
    let t14578 = t1356 * t14498;
    let t14579 = 0.39914139006212695214e-1_f64 * t14578;
    let t14580 = t2144 * t698;
    (t14569, t14570, t14571, t14572, t14574, t14577, t14579, t14580)
}
