//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta934 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3071;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3072;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3073;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta934(t448: f64, t81218: f64, t81250: f64, t300: f64, t1196: f64, t16988: f64, t20895: f64, t20397: f64, t5192: f64, t24488: f64, t3531: f64, t20537: f64, t5197: f64, t20892: f64, t45000: f64, t68255: f64, t68257: f64, t68262: f64, t68277: f64, t81156: f64, t81158: f64, t81162: f64, t81167: f64, t81171: f64, t81175: f64, t81179: f64, t81184: f64, t81188: f64, t81192: f64, t81196: f64, t81200: f64, t81204: f64, t81209: f64, t81214: f64, t43888: f64, t56236: f64, t57872: f64, t57874: f64, t57889: f64, t68332: f64, t68334: f64, t68336: f64, t68389: f64, t68399: f64, t68454: f64, t68456: f64, t81224: f64, t81228: f64, t81230: f64, t81232: f64, t81234: f64, t81236: f64, t81242: f64, t81245: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t81252, t81254, t81257, t81259, t81261, t81264) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3071(t448, t81218, t81250, t300, t1196, t16988, t20895, t20397, t5192, t24488, t3531, t20537, t5197);
        let (t81266, t81286) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3072(t20892, t5192, t45000, t68255, t68257, t68262, t68277, t81156, t81158, t81162, t81167, t81171, t81175, t81179, t81184, t81188, t81192, t81196, t81200, t81204, t81209, t81214);
        let t81304 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3073(t43888, t56236, t57872, t57874, t57889, t68332, t68334, t68336, t68389, t68399, t68454, t68456, t81224, t81228, t81230, t81232, t81234, t81236, t81242, t81245);
    (t81252, t81254, t81257, t81259, t81261, t81264, t81266, t81286, t81304)
}
