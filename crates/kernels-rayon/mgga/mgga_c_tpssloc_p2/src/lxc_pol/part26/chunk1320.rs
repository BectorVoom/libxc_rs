//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1320/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1320(t1081: f64, t2752: f64, t13487: f64, t10121: f64, t28: f64, t2379: f64, t23788: f64, t46240: f64, t25927: f64, t46320: f64, t10140: f64, t3231: f64, t776: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t83555 = t2752 * t1081;
    let t83556 = t83555 * t13487;
    let t83559 = t28 * t10121;
    let t83566 = t1081 * t2379;
    let t83579 = t23788 * t46240;
    let t83582 = t25927 * t46320;
    let t83585 = t28 * t10140;
    let t83592 = t3231 * t776;
    (t83556, t83559, t83566, t83579, t83582, t83585, t83592)
}
