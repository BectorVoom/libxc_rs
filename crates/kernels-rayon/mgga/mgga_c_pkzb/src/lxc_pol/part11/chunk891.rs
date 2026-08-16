//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 891/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk891(t2019: f64, t9674: f64, t2036: f64, t306: f64, t3650: f64, t133: f64, t9539: f64, t793: f64, t2970: f64, t9277: f64, t5931: f64, t9660: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9675 = t2019 * t9674;
    let t9682 = t2036 * t9674;
    let t9685 = t306 * t3650;
    let t9686 = t2019 * t9685;
    let t9691 = t9539 * t133;
    let t9692 = t9691 * t793;
    let t9695 = t2036 * t9685;
    let t9700 = t2970 * t9277;
    let t9703 = t5931 * t9660;
    (t9675, t9682, t9685, t9686, t9691, t9692, t9695, t9700, t9703)
}
