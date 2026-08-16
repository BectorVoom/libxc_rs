//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 681/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk681(t1553: f64, t537: f64, t113: f64, t2115: f64, t1604: f64, t489: f64, t57: f64, t2224: f64, t514: f64, t1620: f64, t2215: f64, t2214: f64, t2232: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5114 = t537 * t1553;
    let t5115 = t5114 * t113;
    let t5116 = t2115 * t5115;
    let t5117 = t1604 * t5116;
    let t5119 = t57 * t489;
    let t5120 = t5119 * t2224;
    let t5121 = t514 * t5120;
    let t5123 = t1620 * t2215;
    let t5125 = t2214 * t2232;
    (t5115, t5116, t5117, t5119, t5121, t5123, t5125)
}
