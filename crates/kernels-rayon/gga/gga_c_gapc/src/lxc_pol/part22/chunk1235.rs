//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1235/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1235(t11322: f64, t611: f64, t9386: f64, t11483: f64, t11485: f64, t1846: f64, t11311: f64, t11317: f64, t1932: f64, t628: f64, t11489: f64, t11316: f64, t3064: f64, t3954: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34666 = t611 * t9386 * t11322;
    let t34669 = t1846 * t11483 * t11485;
    let t34673 = t1932 * t11311 * t11317;
    let t34675 = t628 * t11483;
    let t34676 = t34675 * t11489;
    let t34679 = t11316 * t3064 * t3954;
    (t34666, t34669, t34673, t34675, t34676, t34679)
}
