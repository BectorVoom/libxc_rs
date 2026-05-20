//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta383 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1283;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1284;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1285;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta383<F: Float>(t24839: F, t3720: F, t24232: F, t247: F, t3618: F, t1264: F, t24248: F, t1222: F, t1261: F, t12809: F, t12855: F, t1808: F, t21242: F, t24817: F, t24821: F, t24827: F, t24831: F, t24836: F, t5373: F, t5381: F, t5391: F, t6653: F, t6673: F, t6679: F, t6683: F, t24562: F, t24587: F, t24622: F, t24674: F, t24722: F, t24778: F, t24815: F, t225: F, t494: F, t1210: F, t1274: F, t1775: F, t17995: F, t18059: F, t1829: F, t20697: F, t20700: F, t20753: F, t21394: F, t21621: F, t24509: F, t24515: F, t24519: F, t24525: F, t24698: F, t460: F, t495: F, t5220: F, t5417: F, t6574: F, t6580: F, t6745: F) -> (F, F, F, F, F, F) {
        let (t24840, t24846, t24858, t24861) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1283::<F>(t24839, t3720, t24232, t247, t3618, t1264, t24248, t1222, t1261, t12809, t12855, t1808, t21242, t24817, t24821, t24827, t24831, t24836, t5373, t5381, t5391, t6653, t6673, t6679, t6683);
        let t24864 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1284::<F>(t24562, t24587, t24622, t24674, t24722, t24778, t24815, t24861);
        let (t24866, t24881) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1285::<F>(t225, t24864, t494, t1210, t1274, t1775, t17995, t18059, t1829, t20697, t20700, t20753, t21394, t21621, t24509, t24515, t24519, t24525, t24698, t460, t495, t5220, t5417, t6574, t6580, t6745);
    (t24840, t24846, t24858, t24864, t24866, t24881)
}
