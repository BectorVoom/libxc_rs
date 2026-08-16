//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta438 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1951;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1952;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1953;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta438(t221: f64, t4019: f64, t5659: f64, t4018: f64, t3989: f64, t5629: f64, t3930: f64, t5661: f64, t5665: f64, t9976: f64, t1412: f64, t1882: f64, t3938: f64, t3992: f64, t2661: f64, t1399: f64, t5608: f64, t5651: f64, t10004: f64, t9963: f64, t9971: f64, t9973: f64, t9977: f64, t9982: f64, t13773: f64, t13814: f64, t13860: f64, t13931: f64, t13965: f64, t14002: f64, t14033: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t14036, t14038, t14040, t14042, t14043, t14045) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1951(t221, t4019, t5659, t4018, t3989, t5629, t3930, t5661, t5665, t9976, t1412, t1882);
        let (t14047, t14051, t14055, t14063) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1952(t14045, t3938, t3992, t2661, t1399, t5608, t5651, t10004, t14038, t14040, t14042, t14043, t9963, t9971, t9973, t9977, t9982);
        let t14066 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1953(t13773, t13814, t13860, t13931, t13965, t14002, t14033, t14063);
    (t14036, t14045, t14047, t14051, t14055, t14066)
}
