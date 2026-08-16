//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 1178/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk1178(t11458: f64, t1936: f64, t19670: f64, t11326: f64, t9262: f64, t3688: f64, t8877: f64, t26102: f64, t3709: f64, t11514: f64, t5626: f64, t11488: f64, t1771: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34585 = t19670 * t1936 * t11458;
    let t34587 = t11326 * t9262;
    let t34589 = t3688 * t8877;
    let t34591 = t3709 * t26102;
    let t34593 = t11514 * t5626;
    let t34595 = t11488 * t1771;
    (t34585, t34587, t34589, t34591, t34593, t34595)
}
