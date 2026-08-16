//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 642/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk642(t2364: f64, t2487: f64, t4609: f64, t1876: f64, t4614: f64, t7715: f64, t1877: f64, t7718: f64, t4623: f64, t8504: f64, t706: f64, t7034: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8904 = t4609 * t2364 * t2487;
    let t8908 = t1876 * t4614 * t7715;
    let t8912 = t1876 * t1877 * t7718;
    let t8915 = t4623 * t8504;
    let t8916 = t706 * t8915;
    let t8919 = t7034 * t2487;
    (t8904, t8908, t8912, t8915, t8916, t8919)
}
