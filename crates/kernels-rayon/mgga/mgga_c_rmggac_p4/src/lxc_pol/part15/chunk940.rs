//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 940/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk940(t2160: f64, t638: f64, t9750: f64, t9746: f64, t1525: f64, t236: f64, t618: f64, t7230: f64, t7231: f64, t1818: f64, t495: f64, t9210: f64) -> (f64, f64, f64, f64) {
    let t45633 = t638 * t2160 * t9750;
    let t45636 = t638 * t2160 * t9746;
    let t45641 = t7230 * t7231 * t236 * t618 * t1525;
    let t45646 = t7230 * t9210 * t236 * t1818 * t495;
    (t45633, t45636, t45641, t45646)
}
