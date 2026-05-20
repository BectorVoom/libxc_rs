//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta588 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2218;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2219;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta588<F: Float>(t23598: F, t996: F, t1695: F, t3269: F, t6392: F, t1651: F, t6350: F, t1079: F, t6258: F, t1076: F, t1647: F, t1652: F, t16600: F, t1696: F, t19351: F, t20178: F, t20204: F, t20211: F, t23583: F, t3058: F, t4778: F, t4935: F, t6245: F, t6251: F, t6259: F, t6345: F, t6351: F, t995: F, t1066: F, t23485: F, t247: F, t5819: F, t4801: F, t1042: F, t1668: F, t6305: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t23599, t23603, t23607, t23617, t23621, t23628) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2218::<F>(t23598, t996, t1695, t3269, t6392, t1651, t6350, t1079, t6258, t1076, t1647, t1652, t16600, t1696, t19351, t20178, t20204, t20211, t23583, t3058, t4778, t4935, t6245, t6251, t6259, t6345, t6351, t995);
        let (t23630, t23633, t23634, t23635, t23640) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2219::<F>(t1066, t23485, t247, t1651, t5819, t4801, t1042, t1668, t6305);
    (t23599, t23603, t23607, t23617, t23621, t23628, t23630, t23633, t23634, t23635, t23640)
}
