//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta624 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2310;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2311;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2312;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta624<F: Float>(t1225: F, t22671: F, t1012: F, t13006: F, t22688: F, t13027: F, t13020: F, t1774: F, t6628: F, t3604: F, t3720: F, t3611: F, t24232: F, t247: F, t3618: F, t1264: F, t24248: F, t1222: F, t1261: F, t12809: F, t12855: F, t1808: F, t21242: F, t5373: F, t5381: F, t5391: F, t6653: F, t6673: F, t6679: F, t6683: F, t24562: F, t24587: F, t24622: F, t24674: F, t24722: F, t24778: F, t24815: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t24816, t24817, t24820, t24821, t24826, t24827, t24830, t24831, t24834) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2310::<F>(t1225, t22671, t1012, t13006, t22688, t13027, t13020, t1774, t6628);
        let (t24835, t24836, t24839, t24840, t24846, t24858, t24861) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2311::<F>(t24834, t3604, t3720, t3611, t24232, t247, t3618, t1264, t24248, t1222, t1261, t12809, t12855, t1808, t21242, t24817, t24821, t24827, t24831, t5373, t5381, t5391, t6653, t6673, t6679, t6683);
        let t24864 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2312::<F>(t24562, t24587, t24622, t24674, t24722, t24778, t24815, t24861);
    (t24816, t24820, t24826, t24830, t24834, t24835, t24836, t24839, t24840, t24846, t24858, t24864)
}
