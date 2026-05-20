//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta832 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2693;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2694;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2695;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta832<F: Float>(t1647: F, t16558: F, t1078: F, t6258: F, t3057: F, t6343: F, t3046: F, t20112: F, t342: F, t15669: F, t1678: F, t1679: F, t994: F, t1071: F, t6235: F, t989: F, t20230: F, t3336: F, t2435: F, t6430: F, t6422: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t67972, t68018, t68022, t68072, t68138, t68144, t68170) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2693::<F>(t1647, t16558, t1078, t6258, t3057, t6343, t3046, t20112, t342, t15669, t1678, t1679, t994);
        let (t68185, t68188, t68207, t68255) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2694::<F>(t1071, t6235, t6343, t989, t20230, t3336, t2435, t6430);
        let t68257 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2695::<F>(t2435, t6422);
    (t67972, t68018, t68022, t68072, t68138, t68144, t68170, t68185, t68188, t68207, t68255, t68257)
}
