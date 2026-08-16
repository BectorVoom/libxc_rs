//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 1139/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk1139(t1: f64, t33543: f64, t1084: f64, t33961: f64, t11311: f64, t11791: f64, t2520: f64, t11802: f64, t33490: f64, t11805: f64, t11803: f64, t11804: f64, t19139: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34106 = t33543 * t1;
    let t34108 = t1084 * t34106 * t33961;
    let t34111 = t2520 * t11311 * t11791;
    let t34113 = t11802 * t33490;
    let t34114 = t34113 * t11805;
    let t34117 = t11803 * t11804 * t19139;
    (t34106, t34108, t34111, t34113, t34114, t34117)
}
