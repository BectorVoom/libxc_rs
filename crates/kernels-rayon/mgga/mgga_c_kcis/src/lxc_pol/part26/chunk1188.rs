//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1188/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1188(t26517: f64, t26417: f64, t26632: f64, t782: f64, t826: f64, t26390: f64, t31271: f64, t2585: f64, t740: f64, t7617: f64, t9181: f64, t113: f64, t8538: f64, t9064: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t91785 = 6.0_f64 * t26517;
    let t91786 = 6.0_f64 * t26417;
    let t91789 = t26632 * t782;
    let t91791 = 3.0_f64 * t91789 * t826;
    let t91793 = 18.0_f64 * t31271 * t26390;
    let t91794 = t2585 * t740;
    let t91796 = t9181 * t7617;
    let t91799 = t9064 * t113 * t8538;
    (t91785, t91786, t91791, t91793, t91794, t91796, t91799)
}
