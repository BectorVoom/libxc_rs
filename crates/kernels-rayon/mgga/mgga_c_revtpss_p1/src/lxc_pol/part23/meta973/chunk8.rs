//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3306/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3306(t1450: f64, t23059: f64, t1868: f64, t39528: f64, t39531: f64, t4139: f64, t48234: f64, t48236: f64, t48241: f64, t48244: f64, t75389: f64, t85896: f64, t85897: f64, t85898: f64, t85899: f64) -> (f64, f64) {
    let t86731 = t23059 * t1450;
    let t86741 = 9.0_f64 * t1868 * t4139 * t75389 - t39528 + t39531 + t48234 + t48236 + t48241 - t48244 - t85896 + t85897 - t85898 + t85899;
    (t86731, t86741)
}
