//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta493 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1491;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1492;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta493<F: Float>(t1358: F, t2439: F, t6888: F, t785: F, t1426: F, t6889: F, t786: F, t14090: F, t14100: F, t22427: F, t2435: F, t1432: F, t22379: F, t2470: F, t1437: F, t2482: F, t6843: F, t136: F, t2457: F, t3964: F, t10073: F, t22365: F, t22373: F, t10069: F, t22369: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t74807, t74835, t74838, t74849, t74873) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1491::<F>(t1358, t2439, t6888, t785, t1426, t6889, t786, t14090, t14100, t22427, t2435, t1432, t22379, t2470);
        let (t74892, t74901, t74945, t74990, t74999) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1492::<F>(t1437, t2482, t6843, t136, t2457, t3964, t6888, t10073, t22365, t22373, t10069, t22369);
    (t74807, t74835, t74838, t74849, t74873, t74892, t74901, t74945, t74990, t74999)
}
