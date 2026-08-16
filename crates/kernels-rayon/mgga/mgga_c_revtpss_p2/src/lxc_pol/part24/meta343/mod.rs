//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta343 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1195;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta343(t23598: f64, t996: f64, t1695: f64, t3269: f64, t6392: f64, t1651: f64, t6350: f64, t1079: f64, t6258: f64, t1076: f64, t1647: f64, t1652: f64, t16600: f64, t1696: f64, t19351: f64, t20178: f64, t20204: f64, t20211: f64, t23583: f64, t3058: f64, t4778: f64, t4935: f64, t6245: f64, t6251: f64, t6259: f64, t6345: f64, t6351: f64, t995: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t23599, t23603, t23607, t23616, t23617, t23620, t23621, t23628) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1195(t23598, t996, t1695, t3269, t6392, t1651, t6350, t1079, t6258, t1076, t1647, t1652, t16600, t1696, t19351, t20178, t20204, t20211, t23583, t3058, t4778, t4935, t6245, t6251, t6259, t6345, t6351, t995);
    (t23599, t23603, t23607, t23616, t23617, t23620, t23621, t23628)
}
