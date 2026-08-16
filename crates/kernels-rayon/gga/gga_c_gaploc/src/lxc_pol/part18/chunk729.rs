//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 729/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk729(t2365: f64, t6843: f64, t4391: f64, t4625: f64, t914: f64, t1407: f64, t2467: f64, t1: f64, t6514: f64, t1415: f64, t1391: f64, t2466: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6844 = t2365 * t6843;
    let t6845 = t4391 * t6844;
    let t6847 = t4625 * t914;
    let t6849 = t1407 * t2467;
    let t6851 = t6514 * t1;
    let t6852 = t1415 * t6851;
    let t6855 = t1391 * t2466;
    (t6845, t6847, t6849, t6851, t6852, t6855)
}
