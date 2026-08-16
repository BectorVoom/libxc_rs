//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1028/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1028(t10808: f64, t11141: f64, t224: f64, t1531: f64, t2876: f64, t123: f64, t3338: f64, t2097: f64, t3039: f64, t3431: f64, t5558: f64, t744: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11142 = t10808 + t11141;
    let t11143 = t224 * t11142;
    let t12881 = t2876 * t1531;
    let t12963 = t3338 * t123;
    let t13045 = t3039 * t2097;
    let t13063 = t3431 * t123;
    let t14537 = t744 * t5558;
    (t11142, t11143, t12881, t12963, t13045, t13063, t14537)
}
