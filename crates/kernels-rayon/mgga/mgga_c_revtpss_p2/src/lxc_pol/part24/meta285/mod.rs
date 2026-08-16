//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta285 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1065;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta285(t1678: f64, t3316: f64, t342: f64, t6299: f64, t73: f64, t1065: f64, t6244: f64, t3172: f64, t6301: f64, t1041: f64, t6258: f64, t1032: f64, t6235: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t19607, t19608, t19611, t19649, t19658, t19659, t19675, t19696) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1065(t1678, t3316, t342, t6299, t73, t1065, t6244, t3172, t6301, t1041, t6258, t1032, t6235);
    (t19607, t19608, t19611, t19649, t19658, t19659, t19675, t19696)
}
