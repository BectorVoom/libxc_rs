//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta383 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1283;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1284;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1285;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta383(t24839: f64, t3720: f64, t24232: f64, t247: f64, t3618: f64, t1264: f64, t24248: f64, t1222: f64, t1261: f64, t12809: f64, t12855: f64, t1808: f64, t21242: f64, t24817: f64, t24821: f64, t24827: f64, t24831: f64, t24836: f64, t5373: f64, t5381: f64, t5391: f64, t6653: f64, t6673: f64, t6679: f64, t6683: f64, t24562: f64, t24587: f64, t24622: f64, t24674: f64, t24722: f64, t24778: f64, t24815: f64, t225: f64, t494: f64, t1210: f64, t1274: f64, t1775: f64, t17995: f64, t18059: f64, t1829: f64, t20697: f64, t20700: f64, t20753: f64, t21394: f64, t21621: f64, t24509: f64, t24515: f64, t24519: f64, t24525: f64, t24698: f64, t460: f64, t495: f64, t5220: f64, t5417: f64, t6574: f64, t6580: f64, t6745: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t24840, t24846, t24858, t24861) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1283(t24839, t3720, t24232, t247, t3618, t1264, t24248, t1222, t1261, t12809, t12855, t1808, t21242, t24817, t24821, t24827, t24831, t24836, t5373, t5381, t5391, t6653, t6673, t6679, t6683);
        let t24864 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1284(t24562, t24587, t24622, t24674, t24722, t24778, t24815, t24861);
        let (t24866, t24881) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1285(t225, t24864, t494, t1210, t1274, t1775, t17995, t18059, t1829, t20697, t20700, t20753, t21394, t21621, t24509, t24515, t24519, t24525, t24698, t460, t495, t5220, t5417, t6574, t6580, t6745);
    (t24840, t24846, t24858, t24864, t24866, t24881)
}
