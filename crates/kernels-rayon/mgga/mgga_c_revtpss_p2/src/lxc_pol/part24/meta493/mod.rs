//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta493 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1491;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1492;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta493(t1358: f64, t2439: f64, t6888: f64, t785: f64, t1426: f64, t6889: f64, t786: f64, t14090: f64, t14100: f64, t22427: f64, t2435: f64, t1432: f64, t22379: f64, t2470: f64, t1437: f64, t2482: f64, t6843: f64, t136: f64, t2457: f64, t3964: f64, t10073: f64, t22365: f64, t22373: f64, t10069: f64, t22369: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t74807, t74835, t74838, t74849, t74873) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1491(t1358, t2439, t6888, t785, t1426, t6889, t786, t14090, t14100, t22427, t2435, t1432, t22379, t2470);
        let (t74892, t74901, t74945, t74990, t74999) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1492(t1437, t2482, t6843, t136, t2457, t3964, t6888, t10073, t22365, t22373, t10069, t22369);
    (t74807, t74835, t74838, t74849, t74873, t74892, t74901, t74945, t74990, t74999)
}
