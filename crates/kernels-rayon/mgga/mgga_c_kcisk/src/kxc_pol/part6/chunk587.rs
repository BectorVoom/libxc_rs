//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 587/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk587(t8071: f64, t8184: f64, t504: f64, t2282: f64, t6241: f64, t4170: f64, t196: f64, t7828: f64, t4271: f64, t4272: f64, t7706: f64, t1471: f64, t2059: f64, t6298: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8185 = t8071 + t8184;
    let t8186 = t8185 * t504;
    let t8188 = 2.0_f64 * t6241 * t2282;
    let t8189 = t2282 * t2282;
    let t8191 = 2.0_f64 * t4170 * t8189;
    let t8192 = t7828 * t196;
    let t8212 = t4271 * t4272 * t7706;
    let t8216 = t1471 * t6298 * t2059;
    (t8185, t8186, t8188, t8189, t8191, t8192, t8212, t8216)
}
