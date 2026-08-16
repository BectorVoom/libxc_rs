//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 1054/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk1054(t13483: f64, t8676: f64, t1038: f64, t20602: f64, t3712: f64, t1875: f64, t2972: f64, t134: f64, t8957: f64, t5549: f64, t116: f64, t126: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26778 = t8676 * t13483;
    let t26836 = t3712 * t1038 * t20602;
    let t26887 = t1875 * t2972;
    let t26995 = t8957 * t134;
    let t26996 = t26995 * t5549;
    let t27036 = t116 * t126;
    (t26778, t26836, t26887, t26995, t26996, t27036)
}
