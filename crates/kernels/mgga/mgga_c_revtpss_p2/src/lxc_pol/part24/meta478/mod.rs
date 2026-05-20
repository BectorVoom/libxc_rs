//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta478 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1464;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1465;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1466;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1467;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta478<F: Float>(t1063: F, t11986: F, t247: F, t6096: F, t1086: F, t6343: F, t994: F, t19462: F, t3286: F, t3298: F, t6235: F, t3316: F, t16543: F, t4746: F, t3057: F, t15669: F, t1678: F, t2435: F, t6430: F, t6422: F, t6426: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t67575, t67652, t67714, t67725, t67790) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1464::<F>(t1063, t11986, t247, t6096, t1086, t6343, t994, t19462, t3286, t3298, t6235, t3316);
        let (t67927, t68022, t68144, t68255) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1465::<F>(t16543, t4746, t3057, t6343, t15669, t1678, t2435, t6430);
        let t68257 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1466::<F>(t2435, t6422);
        let t68399 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1467::<F>(t2435, t6426);
    (t67575, t67652, t67714, t67725, t67790, t67927, t68022, t68144, t68255, t68257, t68399)
}
