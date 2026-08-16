//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 1284/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk1284(t297: f64, t825: f64, t10366: f64, t891: f64, t11636: f64, t11688: f64, t6948: f64, t10102: f64, t11620: f64, t1062: f64, t125: f64, t2188: f64, t2536: f64, t329: f64) -> (f64, f64, f64, f64, f64) {
    let t35846 = t825 * t297;
    let t35848 = t10366 * t35846 * t891;
    let t35851 = t11636 * t6948 * t11688;
    let t35853 = t10102 * t11620;
    let t35858 = t1062 * t125 * t2188 * t329 * t2536;
    (t35846, t35848, t35851, t35853, t35858)
}
