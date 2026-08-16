//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta390 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1458;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1459;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1460;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1461;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1462;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta390(t221: f64, t4019: f64, t5659: f64, t4018: f64, t3989: f64, t5629: f64, t3930: f64, t5661: f64, t5665: f64, t9976: f64, t1412: f64, t1882: f64, t3938: f64, t3992: f64, t2661: f64, t1399: f64, t5608: f64, t5651: f64, t10004: f64, t9963: f64, t9971: f64, t9973: f64, t9977: f64, t9982: f64, t13773: f64, t13814: f64, t13860: f64, t13931: f64, t13965: f64, t14002: f64, t14033: f64, t225: f64, t5774: f64, t72: f64, t686: f64, t3915: f64, t5711: f64, t786: f64, t1364: f64, t1357: f64, t5775: f64, t689: f64, t213: f64, t4071: f64, t561: f64, t5728: f64, t9666: f64, t9668: f64, t9672: f64, t9677: f64, t9683: f64, t9687: f64, t9691: f64, t9694: f64, t2470: f64, t5721: f64, t1445: f64, t5599: f64, t2435: f64, t5600: f64, t1426: f64, t1893: f64, t3917: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14036, t14038, t14040, t14042, t14043, t14045) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1458(t221, t4019, t5659, t4018, t3989, t5629, t3930, t5661, t5665, t9976, t1412, t1882);
        let (t14046, t14050, t14054, t14063) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1459(t14045, t3938, t3992, t2661, t1399, t5608, t5651, t10004, t14038, t14040, t14042, t14043, t9963, t9971, t9973, t9977, t9982);
        let (t14066, t14067, t14079) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1460(t13773, t13814, t13860, t13931, t13965, t14002, t14033, t14063, t225, t5774, t72, t686);
        let t14088 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1461(t14079, t3915, t5711, t786, t1364, t1357, t5775, t689, t14067, t213, t4071, t561, t5728, t9666, t9668, t9672, t9677, t9683, t9687, t9691, t9694);
        let (t14090, t14091, t14096, t14097, t14102) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1462(t2470, t5721, t3915, t1445, t5599, t689, t2435, t5600, t1426, t1893, t786, t3917);
    (t14036, t14046, t14050, t14054, t14066, t14079, t14088, t14090, t14091, t14096, t14097, t14102)
}
