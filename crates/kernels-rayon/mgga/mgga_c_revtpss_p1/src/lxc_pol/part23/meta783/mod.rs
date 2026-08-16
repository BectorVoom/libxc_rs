//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta783 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2592;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta783(t1774: f64, t487: f64, t45928: f64, t45934: f64, t45938: f64, t45945: f64, t45949: f64, t2246: f64, t4171: f64, t10308: f64, t1466: f64, t10355: f64, t44: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t60037, t60214, t60215, t60216, t60217, t60218, t60221, t60224, t60308) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2592(t1774, t487, t45928, t45934, t45938, t45945, t45949, t2246, t4171, t10308, t1466, t10355, t44);
    (t60037, t60214, t60215, t60216, t60217, t60218, t60221, t60224, t60308)
}
