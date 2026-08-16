//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta438 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1951;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1952;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1953;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta438<F: Float>(t221: F, t4019: F, t5659: F, t4018: F, t3989: F, t5629: F, t3930: F, t5661: F, t5665: F, t9976: F, t1412: F, t1882: F, t3938: F, t3992: F, t2661: F, t1399: F, t5608: F, t5651: F, t10004: F, t9963: F, t9971: F, t9973: F, t9977: F, t9982: F, t13773: F, t13814: F, t13860: F, t13931: F, t13965: F, t14002: F, t14033: F) -> (F, F, F, F, F, F) {
        let (t14036, t14038, t14040, t14042, t14043, t14045) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1951::<F>(t221, t4019, t5659, t4018, t3989, t5629, t3930, t5661, t5665, t9976, t1412, t1882);
        let (t14047, t14051, t14055, t14063) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1952::<F>(t14045, t3938, t3992, t2661, t1399, t5608, t5651, t10004, t14038, t14040, t14042, t14043, t9963, t9971, t9973, t9977, t9982);
        let t14066 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1953::<F>(t13773, t13814, t13860, t13931, t13965, t14002, t14033, t14063);
    (t14036, t14045, t14047, t14051, t14055, t14066)
}
