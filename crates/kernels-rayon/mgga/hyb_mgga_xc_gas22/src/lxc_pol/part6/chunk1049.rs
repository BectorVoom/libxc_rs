//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1049/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1049(t3014: f64, t545: f64, t9888: f64, t3: f64, t3015: f64, t1890: f64, t3840: f64, t1897: f64, t3804: f64, t3008: f64, t1802: f64, t8587: f64, t8588: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9890 = t3014 * t9888 * t545;
    let t9894 = t3014 * t3015 * t3;
    let t9897 = t1890 * t3840;
    let t9899 = t1897 * t3804;
    let t9901 = t3008 * t9899 * t545;
    let t9904 = t1802 * t3804;
    let t9906 = t3014 * t9904 * t545;
    let t9909 = -t8587 - t8588;
    (t9890, t9894, t9897, t9899, t9901, t9904, t9906, t9909)
}
