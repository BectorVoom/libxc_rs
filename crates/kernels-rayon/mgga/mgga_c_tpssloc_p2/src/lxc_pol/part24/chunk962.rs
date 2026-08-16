//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 962/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk962(t10277: f64, t2978: f64, t9288: f64, t974: f64, t1030: f64, t363: f64, t3068: f64, t1058: f64, t10213: f64, t10216: f64, t3030: f64, t990: f64) -> (f64, f64, f64, f64) {
    let t10930 = t2978 * t10277;
    let t10931 = t10930 * t9288;
    let t10932 = t974 * t10931;
    let t10935 = t363 * t1030;
    let t10936 = t10935 * t3068;
    let t10937 = t1058 * t10936;
    let t10942 = t10213 * t10216;
    let t10943 = t10942 * t9288;
    let t10944 = t974 * t10943;
    let t10947 = t990 * t3030;
    (t10932, t10937, t10944, t10947)
}
