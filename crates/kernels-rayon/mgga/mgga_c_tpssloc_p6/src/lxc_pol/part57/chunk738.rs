//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 738/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk738(t22779: f64, t7712: f64, t1887: f64, t22839: f64, t1377: f64, t1799: f64, t22674: f64, t7700: f64, t6897: f64, t6883: f64, t7697: f64, t225: f64, t7723: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26295 = t22779 * t7712;
    let t26331 = t22839 * t1887;
    let t26337 = t1377 * t1799;
    let t26344 = t22674 * t7700;
    let t26345 = t6897 * t26344;
    let t26361 = t6883 * t7697;
    let t26366 = t7723 * t225;
    (t26295, t26331, t26337, t26345, t26361, t26366)
}
