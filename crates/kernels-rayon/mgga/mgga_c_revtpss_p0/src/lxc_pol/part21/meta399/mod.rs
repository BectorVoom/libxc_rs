//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta399 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1857;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta399(t1214: f64, t2258: f64, t5296: f64, t1042: f64, t3617: f64, t3363: f64, t3172: f64, t3590: f64, t1247: f64, t11231: f64, t5302: f64, t3612: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12931, t12932, t12933, t12937, t12938, t12941, t12942, t12944, t12945, t12948) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1857(t1214, t2258, t5296, t1042, t3617, t3363, t3172, t3590, t1247, t11231, t5302, t3612);
    (t12931, t12932, t12933, t12937, t12938, t12941, t12942, t12944, t12945, t12948)
}
