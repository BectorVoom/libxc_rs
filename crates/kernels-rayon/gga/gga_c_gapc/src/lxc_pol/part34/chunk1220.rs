//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1220/1427 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1220(t12744: f64, t5407: f64, t9113: f64, t1643: f64, t22327: f64, t3679: f64, t1266: f64, t205: f64, t3683: f64, t144: f64, t3095: f64, t3094: f64, t3954: f64) -> (f64, f64, f64, f64, f64) {
    let t34436 = t9113 * t12744 * t5407;
    let t34439 = t1643 * t3679 * t22327;
    let t34442 = t1266 * t3683 * t205;
    let t34447 = t3095 * t144;
    let t34449 = t3094 * t34447 * t3954;
    (t34436, t34439, t34442, t34447, t34449)
}
