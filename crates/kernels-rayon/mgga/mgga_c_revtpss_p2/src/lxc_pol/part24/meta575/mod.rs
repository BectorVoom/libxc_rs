//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta575 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1759;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1760;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1761;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta575(t1723: f64, t81513: f64, t20356: f64, t6449: f64, t20365: f64, t24312: f64, t5087: f64, t56236: f64, t58153: f64, t68399: f64, t68583: f64, t68585: f64, t68590: f64, t81236: f64, t81491: f64, t81496: f64, t81539: f64, t90400: f64, t90456: f64, t90478: f64, t1179: f64, t1188: f64, t1196: f64, t6474: f64, t68952: f64, t90349: f64, t90351: f64, t90356: f64, t90361: f64, t90364: f64, t90367: f64, t90370: f64, t90373: f64, t90375: f64, t90377: f64, t24324: f64, t5063: f64, t24327: f64, t58473: f64, t12230: f64, t44017: f64, t90324: f64, t68255: f64, t68257: f64, t81156: f64, t81158: f64, t89839: f64, t89851: f64, t89865: f64, t89869: f64, t89873: f64, t89877: f64, t90379: f64, t90384: f64, t90387: f64, t90390: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t90486, t90488, t90490, t90492, t90497) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1759(t1723, t81513, t20356, t6449, t20365, t24312, t5087, t56236, t58153, t68399, t68583, t68585, t68590, t81236, t81491, t81496, t81539);
        let (t90499, t90503, t90505, t90506) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1760(t90400, t90456, t90478, t90497, t1179, t1188, t1196, t6474, t68952, t90349, t90351, t90356, t90361, t90364, t90367, t90370, t90373, t90375, t90377);
        let (t90509, t90511, t90514, t90529) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1761(t24324, t5063, t24327, t58473, t12230, t44017, t90324, t68255, t68257, t81156, t81158, t89839, t89851, t89865, t89869, t89873, t89877, t90379, t90384, t90387, t90390);
    (t90486, t90488, t90490, t90492, t90499, t90503, t90505, t90506, t90509, t90511, t90514, t90529)
}
