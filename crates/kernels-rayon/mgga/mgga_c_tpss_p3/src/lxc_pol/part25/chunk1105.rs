//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1105/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1105(t5107: f64, t673: f64, t5110: f64, t1013: f64, t15235: f64, t128: f64) -> (f64, f64, f64) {
    let t15245 = t673 * t5107;
    let t15248 = t673 * t5110;
    let t15250 = t1013 * t15235;
    let t15251 = t128 * t15250;
    (t15245, t15248, t15251)
}
