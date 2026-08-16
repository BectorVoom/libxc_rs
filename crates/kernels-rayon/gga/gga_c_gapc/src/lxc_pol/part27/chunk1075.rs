//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 1075/1310 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk1075(t4043: f64, t519: f64, t1084: f64, t9419: f64, t11791: f64, t3382: f64, t129: f64, t18551: f64, t18553: f64, t3284: f64, t1086: f64, t11311: f64, t23466: f64, t7624: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t33311 = t519 * t4043;
    let t33312 = t1084 * t33311;
    let t33313 = t33312 * t9419;
    let t33315 = t3382 * t11791;
    let t33320 = t18551 * t129 * t3284 * t18553;
    let t33324 = t7624 * t11311 * t1086 * t23466;
    (t33311, t33312, t33313, t33315, t33320, t33324)
}
