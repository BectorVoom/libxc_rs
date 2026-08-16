//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 1194/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk1194(t1036: f64, t11316: f64, t15341: f64, t1030: f64, t12768: f64, t1749: f64, t11438: f64, t21649: f64, t3021: f64, t1649: f64, t33303: f64, t5553: f64) -> (f64, f64, f64, f64, f64) {
    let t34785 = t11316 * t1036 * t15341;
    let t34788 = t1030 * t12768 * t1749;
    let t34791 = t11438 * t3021 * t21649;
    let t34793 = t33303 * t1649;
    let t34794 = t5553 * t34793;
    (t34785, t34788, t34791, t34793, t34794)
}
