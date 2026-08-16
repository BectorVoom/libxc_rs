//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 704/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk704(t1155: f64, t3403: f64, t3439: f64, t60: f64, t461: f64, t3448: f64, t457: f64, t974: f64, t1229: f64, t3247: f64, t1215: f64, t3508: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4883 = t3403 * t1155;
    let t4899 = t60 * t3439;
    let t4900 = t4899 * t461;
    let t4908 = t3448 * t461;
    let t4934 = t974 * t457;
    let t4972 = t1229 * t3247;
    let t4978 = t3508 * t1215;
    (t4883, t4899, t4900, t4908, t4934, t4972, t4978)
}
