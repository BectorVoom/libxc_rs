//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1194/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1194(t32073: f64, t2317: f64, t6525: f64, t8026: f64, t1365: f64, t23983: f64, t25575: f64, t4382: f64, t986: f64, t6470: f64, t9074: f64, t1016: f64, t21438: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t32074 = 0.23712505529730124666e-2_f64 * t32073;
    let t32076 = t6525 * t8026 * t2317;
    let t32077 = 0.23712505529730124666e-2_f64 * t32076;
    let t32079 = t23983 * t1365 * t25575;
    let t32080 = 0.23712505529730124666e-2_f64 * t32079;
    let t32081 = t4382 * t986;
    let t32083 = t9074 * t32081 * t6470;
    let t32084 = 0.82993769354055436331e-2_f64 * t32083;
    let t32091 = t21438 * t1016;
    (t32074, t32077, t32080, t32081, t32084, t32091)
}
