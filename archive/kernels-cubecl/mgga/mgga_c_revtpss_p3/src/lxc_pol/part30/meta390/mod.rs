//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta390 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1458;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1459;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1460;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1461;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1462;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta390<F: Float>(t221: F, t4019: F, t5659: F, t4018: F, t3989: F, t5629: F, t3930: F, t5661: F, t5665: F, t9976: F, t1412: F, t1882: F, t3938: F, t3992: F, t2661: F, t1399: F, t5608: F, t5651: F, t10004: F, t9963: F, t9971: F, t9973: F, t9977: F, t9982: F, t13773: F, t13814: F, t13860: F, t13931: F, t13965: F, t14002: F, t14033: F, t225: F, t5774: F, t72: F, t686: F, t3915: F, t5711: F, t786: F, t1364: F, t1357: F, t5775: F, t689: F, t213: F, t4071: F, t561: F, t5728: F, t9666: F, t9668: F, t9672: F, t9677: F, t9683: F, t9687: F, t9691: F, t9694: F, t2470: F, t5721: F, t1445: F, t5599: F, t2435: F, t5600: F, t1426: F, t1893: F, t3917: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t14036, t14038, t14040, t14042, t14043, t14045) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1458::<F>(t221, t4019, t5659, t4018, t3989, t5629, t3930, t5661, t5665, t9976, t1412, t1882);
        let (t14046, t14050, t14054, t14063) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1459::<F>(t14045, t3938, t3992, t2661, t1399, t5608, t5651, t10004, t14038, t14040, t14042, t14043, t9963, t9971, t9973, t9977, t9982);
        let (t14066, t14067, t14079) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1460::<F>(t13773, t13814, t13860, t13931, t13965, t14002, t14033, t14063, t225, t5774, t72, t686);
        let t14088 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1461::<F>(t14079, t3915, t5711, t786, t1364, t1357, t5775, t689, t14067, t213, t4071, t561, t5728, t9666, t9668, t9672, t9677, t9683, t9687, t9691, t9694);
        let (t14090, t14091, t14096, t14097, t14102) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1462::<F>(t2470, t5721, t3915, t1445, t5599, t689, t2435, t5600, t1426, t1893, t786, t3917);
    (t14036, t14046, t14050, t14054, t14066, t14079, t14088, t14090, t14091, t14096, t14097, t14102)
}
