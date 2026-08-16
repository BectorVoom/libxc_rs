//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 765/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk765(t2746: f64, t8786: f64, t882: f64, t2709: f64, t8630: f64, t864: f64, t8640: f64, t8646: f64, t8649: f64, t8653: f64, t8660: f64, t8666: f64, t8669: f64, t867: f64, t8674: f64, t8678: f64, t8717: f64) -> (f64, f64) {
    let t8788 = t8786 * t882 * t2746;
    let t8797 = t8630 - 0.1025389702100779493e4_f64 * t867 * t8660 + t8646 - t8649 - t8653 - 0.56969282336565386482e-3_f64 * t864 * t8717 + 0.48159446095139119799e0_f64 * t2709 * t8640 + t8666 - t8669 + t8674 + t8678;
    (t8788, t8797)
}
