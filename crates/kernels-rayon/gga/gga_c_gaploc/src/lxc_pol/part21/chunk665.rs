//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 665/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk665(t5679: f64, t789: f64, t1: f64, t5501: f64, t787: f64, t4371: f64, t734: f64, t2066: f64, t796: f64, t4752: f64, t702: f64, t1645: f64, t1836: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5680 = t5679 * t789;
    let t5687 = t5501 * t1;
    let t5688 = t787 * t5687;
    let t5694 = t4371 * t734;
    let t5703 = t2066 * t796;
    let t5715 = t4752 * t702;
    let t5724 = t1645 * t1836;
    (t5680, t5688, t5694, t5703, t5715, t5724)
}
