//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta314 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1101;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta314(t6862: f64, t72: f64, t686: f64, t10023: f64, t1385: f64, t6888: f64, t14239: f64, t5741: f64, t6844: f64, t4101: f64, t6874: f64, t545: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22314, t22315, t22316, t22321, t22329, t22331, t22332, t22333, t22335, t22336, t22337, t22351) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1101(t6862, t72, t686, t10023, t1385, t6888, t14239, t5741, t6844, t4101, t6874, t545);
    (t22314, t22315, t22316, t22321, t22329, t22331, t22332, t22333, t22335, t22336, t22337, t22351)
}
