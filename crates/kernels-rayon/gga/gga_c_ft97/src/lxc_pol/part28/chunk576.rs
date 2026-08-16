//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 576/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk576(t444: f64, t5811: f64, t2001: f64, t129: f64, t1691: f64, t14: f64, t549: f64, t72: f64, t5828: f64, t542: f64, t550: f64, t133: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t23714 = t5811 * t444;
    let t23715 = t2001 * t23714;
    let t23721 = t129 * t1691;
    let t23724 = t549 * t14;
    let t23725 = t23724 * t72;
    let t23732 = t2001 * t5828;
    let t23742 = t542 * t550;
    let t23745 = t133 * t550;
    (t23714, t23715, t23721, t23724, t23725, t23732, t23742, t23745)
}
