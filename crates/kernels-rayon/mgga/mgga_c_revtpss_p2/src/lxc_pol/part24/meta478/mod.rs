//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta478 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1464;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1465;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1466;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1467;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta478(t1063: f64, t11986: f64, t247: f64, t6096: f64, t1086: f64, t6343: f64, t994: f64, t19462: f64, t3286: f64, t3298: f64, t6235: f64, t3316: f64, t16543: f64, t4746: f64, t3057: f64, t15669: f64, t1678: f64, t2435: f64, t6430: f64, t6422: f64, t6426: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t67575, t67652, t67714, t67725, t67790) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1464(t1063, t11986, t247, t6096, t1086, t6343, t994, t19462, t3286, t3298, t6235, t3316);
        let (t67927, t68022, t68144, t68255) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1465(t16543, t4746, t3057, t6343, t15669, t1678, t2435, t6430);
        let t68257 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1466(t2435, t6422);
        let t68399 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1467(t2435, t6426);
    (t67575, t67652, t67714, t67725, t67790, t67927, t68022, t68144, t68255, t68257, t68399)
}
