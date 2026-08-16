//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 1198/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk1198(t15466: f64, t15512: f64, t15558: f64, t15601: f64, t15648: f64, t15684: f64, t15726: f64, t15768: f64, t493: f64, t1215: f64, t5052: f64, t1246: f64) -> (f64, f64, f64) {
    let t15771 = t15466 + t15512 + t15558 + t15601 + t15648 + t15684 + t15726 + t15768;
    let t15772 = t493 * t15771;
    let t15776 = t5052 * t1215;
    let t15777 = t15776 * t1246;
    (t15771, t15772, t15777)
}
