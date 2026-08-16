//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta551 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2108;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta551(t6862: f64, t72: f64, t686: f64, t10023: f64, t1385: f64, t6888: f64, t10070: f64, t10074: f64, t1399: f64, t14191: f64, t14193: f64, t14203: f64, t14209: f64, t14255: f64, t1883: f64, t213: f64, t21981: f64, t22005: f64, t22009: f64, t22016: f64, t22307: f64, t4118: f64, t546: f64, t5659: f64, t5675: f64, t5745: f64, t5755: f64, t5767: f64, t6874: f64, t820: f64) -> (f64, f64, f64, f64, f64) {
        let (t22314, t22315, t22316, t22321, t22325) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2108(t6862, t72, t686, t10023, t1385, t6888, t10070, t10074, t1399, t14191, t14193, t14203, t14209, t14255, t1883, t213, t21981, t22005, t22009, t22016, t22307, t4118, t546, t5659, t5675, t5745, t5755, t5767, t6874, t820);
    (t22314, t22315, t22316, t22321, t22325)
}
