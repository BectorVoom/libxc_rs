//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 689/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk689(t317: f64, t7021: f64, t684: f64, t2665: f64, t1091: f64, t25465: f64, t25446: f64, t1465: f64, t3051: f64, t3746: f64, t6217: f64, t10683: f64, t4162: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t28985 = t7021 * t317;
    let t28986 = t28985 * t684;
    let t28987 = t2665 * t28986;
    let t28992 = t25465 * t1091;
    let t28993 = t2665 * t28992;
    let t28997 = t2665 * t25446 * t1091;
    let t29000 = t1465 * t3051;
    let t29002 = t2665 * t6217 * t3746;
    let t29006 = t10683 * t6217 * t4162;
    (t28985, t28986, t28987, t28992, t28993, t28997, t29000, t29002, t29006)
}
