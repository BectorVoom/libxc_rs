//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1050/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1050(t2626: f64, t7609: f64, t113: f64, t2585: f64, t2588: f64, t7617: f64, t740: f64, t805: f64, t2491: f64, t2593: f64, t774: f64, t808: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t26520 = t7609 * t2626;
    let t26521 = t2585 * t113;
    let t26523 = t2588 * t7617;
    let t26525 = t805 * t740;
    let t26527 = t113 * t2491;
    let t26528 = t2593 * t26527;
    let t26530 = t740 * t774;
    let t26531 = t808 * t26530;
    (t26520, t26521, t26523, t26525, t26527, t26528, t26530, t26531)
}
