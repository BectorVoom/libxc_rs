//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2579/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2579(t1179: f64, t1188: f64, t20382: f64, t1196: f64, t5192: f64, t5202: f64, t5207: f64, t1189: f64, t6555: f64, t5181: f64, t5197: f64, t16988: f64, t5205: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t20384 = t1179 * t20382 * t1188;
    let t20386 = 0.5848223622634646207e0_f64 * t1196 * t20384;
    let t20388 = 0.11696447245269292414e1_f64 * t5192 * t5202;
    let t20390 = 0.34631718211362927517e2_f64 * t5192 * t5207;
    let t20391 = t6555 * t1189;
    let t20393 = 0.35089341735807877242e1_f64 * t1196 * t20391;
    let t20394 = t5197 * t5181;
    let t20396 = 0.23392894490538584828e1_f64 * t1196 * t20394;
    let t20397 = t5205 * t16988;
    (t20384, t20386, t20388, t20390, t20391, t20393, t20394, t20396, t20397)
}
