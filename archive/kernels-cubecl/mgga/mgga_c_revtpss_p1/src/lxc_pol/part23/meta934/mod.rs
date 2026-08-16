//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta934 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3071;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3072;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3073;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta934<F: Float>(t448: F, t81218: F, t81250: F, t300: F, t1196: F, t16988: F, t20895: F, t20397: F, t5192: F, t24488: F, t3531: F, t20537: F, t5197: F, t20892: F, t45000: F, t68255: F, t68257: F, t68262: F, t68277: F, t81156: F, t81158: F, t81162: F, t81167: F, t81171: F, t81175: F, t81179: F, t81184: F, t81188: F, t81192: F, t81196: F, t81200: F, t81204: F, t81209: F, t81214: F, t43888: F, t56236: F, t57872: F, t57874: F, t57889: F, t68332: F, t68334: F, t68336: F, t68389: F, t68399: F, t68454: F, t68456: F, t81224: F, t81228: F, t81230: F, t81232: F, t81234: F, t81236: F, t81242: F, t81245: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t81252, t81254, t81257, t81259, t81261, t81264) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3071::<F>(t448, t81218, t81250, t300, t1196, t16988, t20895, t20397, t5192, t24488, t3531, t20537, t5197);
        let (t81266, t81286) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3072::<F>(t20892, t5192, t45000, t68255, t68257, t68262, t68277, t81156, t81158, t81162, t81167, t81171, t81175, t81179, t81184, t81188, t81192, t81196, t81200, t81204, t81209, t81214);
        let t81304 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3073::<F>(t43888, t56236, t57872, t57874, t57889, t68332, t68334, t68336, t68389, t68399, t68454, t68456, t81224, t81228, t81230, t81232, t81234, t81236, t81242, t81245);
    (t81252, t81254, t81257, t81259, t81261, t81264, t81266, t81286, t81304)
}
