//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1501/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1501(t10277: f64, t2978: f64, t9288: f64, t974: f64, t1030: f64, t363: f64, t3068: f64, t1058: f64) -> (f64, f64, f64, f64, f64) {
    let t10930 = t2978 * t10277;
    let t10931 = t10930 * t9288;
    let t10932 = t974 * t10931;
    let t10935 = t363 * t1030;
    let t10936 = t10935 * t3068;
    let t10937 = t1058 * t10936;
    (t10931, t10932, t10935, t10936, t10937)
}
