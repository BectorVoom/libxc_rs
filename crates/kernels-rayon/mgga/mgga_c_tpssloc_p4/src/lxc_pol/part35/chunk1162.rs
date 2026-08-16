//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1162/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1162(t22759: f64, t242: f64, t1336: f64, t1887: f64, t22839: f64, t1799: f64, t567: f64, t1377: f64, t22674: f64, t7700: f64, t6897: f64, t1842: f64, t3886: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t26308 = t22759 * t242;
    let t26309 = t1336 * t26308;
    let t26331 = t22839 * t1887;
    let t26332 = t567 * t1799;
    let t26337 = t1377 * t1799;
    let t26344 = t22674 * t7700;
    let t26345 = t6897 * t26344;
    let t26354 = t3886 * t1842;
    (t26309, t26331, t26332, t26337, t26344, t26345, t26354)
}
