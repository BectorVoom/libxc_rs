//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta544 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2207;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2208;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2209;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta544<F: Float>(t1757: F, t3515: F, t3497: F, t5184: F, t3523: F, t5180: F, t1187: F, t12429: F, t12470: F, t12481: F, t12486: F, t12491: F, t16955: F, t16959: F, t16962: F, t16966: F, t16971: F, t16974: F, t16979: F, t3477: F, t3496: F, t3521: F, t5163: F, t5185: F, t12555: F, t1756: F, t16710: F, t16712: F, t12297: F, t12299: F, t12301: F, t12303: F, t12397: F, t16706: F, t16708: F, t16717: F, t16722: F, t16727: F, t16731: F, t16735: F, t16740: F, t16744: F, t16748: F, t1737: F, t3451: F, t1160: F, t5117: F, t1170: F, t12511: F, t12553: F, t16809: F, t16832: F, t16837: F, t16839: F, t16842: F, t16844: F, t16846: F, t16945: F, t3454: F, t435: F, t5125: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t16982, t16985, t16988, t16989, t16992, t16995) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2207::<F>(t1757, t3515, t3497, t5184, t3523, t5180, t1187, t12429, t12470, t12481, t12486, t12491, t16955, t16959, t16962, t16966, t16971, t16974, t16979, t3477, t3496, t3521, t5163, t5185);
        let (t16997, t16998, t17020) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2208::<F>(t12555, t1756, t3497, t16710, t16712, t12297, t12299, t12301, t12303, t12397, t16706, t16708, t16717, t16722, t16727, t16731, t16735, t16740, t16744, t16748);
        let (t17023, t17026, t17029) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2209::<F>(t1737, t3451, t1160, t5117, t1170, t12511, t12553, t16809, t16832, t16837, t16839, t16842, t16844, t16846, t16945, t16998, t17020, t3454, t435, t5125);
    (t16982, t16985, t16988, t16989, t16992, t16995, t16997, t16998, t17020, t17023, t17026, t17029)
}
