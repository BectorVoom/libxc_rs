//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 1141/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk1141(t11802: f64, t33490: f64, t11805: f64, t11803: f64, t11804: f64, t19139: f64, t33560: f64, t9419: f64, t11808: f64, t29516: f64, t3707: f64, t4780: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34113 = t11802 * t33490;
    let t34114 = t34113 * t11805;
    let t34117 = t11803 * t11804 * t19139;
    let t34119 = t33560 * t9419;
    let t34121 = t11808 * t29516;
    let t34123 = t4780 * t3707;
    (t34113, t34114, t34117, t34119, t34121, t34123)
}
