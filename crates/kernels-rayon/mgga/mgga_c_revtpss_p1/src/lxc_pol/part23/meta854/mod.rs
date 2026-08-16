//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta854 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2741;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2742;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta854(t17290: f64, t5362: f64, t17435: f64, t5327: f64, t3655: f64, t6595: f64, t1256: f64, t21313: f64, t21316: f64, t1261: f64, t20272: f64, t247: f64, t3634: f64, t12916: f64, t20951: f64, t5340: f64, t17396: f64, t17620: f64, t17472: f64, t5373: f64, t1222: f64, t17471: f64, t20266: f64, t17351: f64, t20770: f64, t56756: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t71740, t71742, t71744, t71749, t71751, t71827) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2741(t17290, t5362, t17435, t5327, t3655, t6595, t1256, t21313, t21316, t1261, t20272, t247, t3634);
        let (t71845, t71859, t71880, t71883, t71886) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2742(t12916, t20951, t5340, t17396, t17620, t17472, t5373, t1222, t17471, t20266, t17351, t20770, t56756);
    (t71740, t71742, t71744, t71749, t71751, t71827, t71845, t71859, t71880, t71883, t71886)
}
