//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta358 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1712;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1713;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1714;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta358<F: Float>(t11659: F, t4910: F, t3117: F, t1016: F, t697: F, t1011: F, t1010: F, t2270: F, t3241: F, t3244: F, t1058: F, t3197: F, t11132: F, t11134: F, t11136: F, t11138: F, t11140: F, t11147: F, t11153: F, t11158: F, t11162: F, t11167: F, t11171: F, t341: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t11876, t11877, t11880, t11881, t11883) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1712::<F>(t11659, t4910, t3117, t1016, t697, t1011, t1010, t2270);
        let (t11886, t11888, t11890, t11901) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1713::<F>(t3241, t3244, t1058, t3197, t11132, t11134, t11136, t11138, t11140, t11147, t11153, t11158, t11162, t11167, t11171);
        let t11902 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1714::<F>(t11901, t341);
    (t11876, t11877, t11880, t11881, t11883, t11886, t11888, t11890, t11901, t11902)
}
