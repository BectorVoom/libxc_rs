//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta575 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1759;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1760;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1761;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta575<F: Float>(t1723: F, t81513: F, t20356: F, t6449: F, t20365: F, t24312: F, t5087: F, t56236: F, t58153: F, t68399: F, t68583: F, t68585: F, t68590: F, t81236: F, t81491: F, t81496: F, t81539: F, t90400: F, t90456: F, t90478: F, t1179: F, t1188: F, t1196: F, t6474: F, t68952: F, t90349: F, t90351: F, t90356: F, t90361: F, t90364: F, t90367: F, t90370: F, t90373: F, t90375: F, t90377: F, t24324: F, t5063: F, t24327: F, t58473: F, t12230: F, t44017: F, t90324: F, t68255: F, t68257: F, t81156: F, t81158: F, t89839: F, t89851: F, t89865: F, t89869: F, t89873: F, t89877: F, t90379: F, t90384: F, t90387: F, t90390: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t90486, t90488, t90490, t90492, t90497) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1759::<F>(t1723, t81513, t20356, t6449, t20365, t24312, t5087, t56236, t58153, t68399, t68583, t68585, t68590, t81236, t81491, t81496, t81539);
        let (t90499, t90503, t90505, t90506) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1760::<F>(t90400, t90456, t90478, t90497, t1179, t1188, t1196, t6474, t68952, t90349, t90351, t90356, t90361, t90364, t90367, t90370, t90373, t90375, t90377);
        let (t90509, t90511, t90514, t90529) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1761::<F>(t24324, t5063, t24327, t58473, t12230, t44017, t90324, t68255, t68257, t81156, t81158, t89839, t89851, t89865, t89869, t89873, t89877, t90379, t90384, t90387, t90390);
    (t90486, t90488, t90490, t90492, t90499, t90503, t90505, t90506, t90509, t90511, t90514, t90529)
}
