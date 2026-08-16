//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta548 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2099;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2100;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2101;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta548<F: Float>(t22190: F, t22203: F, t22210: F, t22220: F, t225: F, t1877: F, t73: F, t4010: F, t6836: F, t1353: F, t5591: F, t5651: F, t1412: F, t6816: F, t1394: F, t21969: F, t1392: F, t1395: F, t1879: F, t539: F, t541: F, t5644: F, t5650: F, t5652: F, t5655: F, t6832: F, t6837: F, t6840: F, t543: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t22223, t22229, t22236, t22237, t22240) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2099::<F>(t22190, t22203, t22210, t22220, t225, t1877, t73, t4010, t6836, t1353, t5591, t5651);
        let (t22245, t22246, t22249, t22252) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2100::<F>(t1412, t6816, t1353, t1394, t21969, t1392, t1395, t1877, t1879, t22223, t22229, t22237, t22240, t539, t541, t5644, t5650, t5652, t5655, t6832, t6837, t6840);
        let t22253 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2101::<F>(t22252, t543);
    (t22223, t22229, t22236, t22237, t22240, t22245, t22246, t22249, t22252, t22253)
}
