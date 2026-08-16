//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 932/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk932(t10213: f64, t10216: f64, t9288: f64, t974: f64, t3030: f64, t990: f64, t3032: f64, t3129: f64, t3038: f64, t3087: f64, t372: f64, t364: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10942 = t10213 * t10216;
    let t10943 = t10942 * t9288;
    let t10944 = t974 * t10943;
    let t10947 = t990 * t3030;
    let t10948 = t10947 * t3032;
    let t10949 = t10948 * t3129;
    let t10952 = t10948 * t3038;
    let t10955 = t3087 * t372;
    let t10956 = t364 * t10955;
    (t10943, t10944, t10947, t10948, t10949, t10952, t10956)
}
