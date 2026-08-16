//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta624 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2310;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2311;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2312;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta624(t1225: f64, t22671: f64, t1012: f64, t13006: f64, t22688: f64, t13027: f64, t13020: f64, t1774: f64, t6628: f64, t3604: f64, t3720: f64, t3611: f64, t24232: f64, t247: f64, t3618: f64, t1264: f64, t24248: f64, t1222: f64, t1261: f64, t12809: f64, t12855: f64, t1808: f64, t21242: f64, t5373: f64, t5381: f64, t5391: f64, t6653: f64, t6673: f64, t6679: f64, t6683: f64, t24562: f64, t24587: f64, t24622: f64, t24674: f64, t24722: f64, t24778: f64, t24815: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t24816, t24817, t24820, t24821, t24826, t24827, t24830, t24831, t24834) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2310(t1225, t22671, t1012, t13006, t22688, t13027, t13020, t1774, t6628);
        let (t24835, t24836, t24839, t24840, t24846, t24858, t24861) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2311(t24834, t3604, t3720, t3611, t24232, t247, t3618, t1264, t24248, t1222, t1261, t12809, t12855, t1808, t21242, t24817, t24821, t24827, t24831, t5373, t5381, t5391, t6653, t6673, t6679, t6683);
        let t24864 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2312(t24562, t24587, t24622, t24674, t24722, t24778, t24815, t24861);
    (t24816, t24820, t24826, t24830, t24834, t24835, t24836, t24839, t24840, t24846, t24858, t24864)
}
