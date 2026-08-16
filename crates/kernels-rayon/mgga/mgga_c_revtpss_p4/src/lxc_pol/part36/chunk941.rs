//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 941/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk941(t22361: f64, t2782: f64, t21981: f64, t4086: f64, t543: f64, t22009: f64, t22005: f64, t6888: f64, t72: f64, t1432: f64, t686: f64, t213: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22362 = t2782 * t22361;
    let t22365 = t4086 * t21981 * t543;
    let t22366 = t2782 * t22365;
    let t22369 = t4086 * t22009 * t543;
    let t22370 = t2782 * t22369;
    let t22373 = t4086 * t22005 * t543;
    let t22374 = t2782 * t22373;
    let t22379 = t6888 * t72;
    let t22381 = t1432 * t22379 * t686;
    let t22390 = t213 * t6888;
    (t22362, t22366, t22370, t22374, t22381, t22390)
}
