//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta578 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2188;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2189;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta578(t23279: f64, t2477: f64, t828: f64, t23177: f64, t827: f64, t23245: f64, t18426: f64, t2747: f64, t6035: f64, t4364: f64, t4365: f64, t6017: f64, t14586: f64, t18444: f64, t10756: f64, t10758: f64, t14780: f64, t14817: f64, t14820: f64, t14839: f64, t18350: f64, t18354: f64, t2745: f64, t4362: f64, t825: f64, t851: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t23281, t23285, t23289, t23293, t23297) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2188(t23279, t2477, t828, t23177, t827, t23245, t18426, t2747, t6035, t4364, t4365, t6017);
        let (t23301, t23310) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2189(t14586, t18444, t4364, t10756, t10758, t14780, t14817, t14820, t14839, t18350, t18354, t23281, t23285, t23289, t23293, t23297, t2745, t4362, t825, t851);
    (t23281, t23285, t23289, t23293, t23297, t23301, t23310)
}
