//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 1188/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk1188(t11311: f64, t11317: f64, t1932: f64, t11483: f64, t628: f64, t11489: f64, t11316: f64, t3064: f64, t3954: f64, t1030: f64, t33303: f64, t3123: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34673 = t1932 * t11311 * t11317;
    let t34675 = t628 * t11483;
    let t34676 = t34675 * t11489;
    let t34679 = t11316 * t3064 * t3954;
    let t34681 = t1030 * t33303;
    let t34682 = t34681 * t3123;
    (t34673, t34675, t34676, t34679, t34681, t34682)
}
