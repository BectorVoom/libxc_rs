//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta367 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1420;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta367(t1226: f64, t4965: f64, t11697: f64, t4953: f64, t3577: f64, t1229: f64, t3242: f64, t13969: f64, t4979: f64, t3506: f64, t4973: f64, t1227: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15594, t15608, t15610, t15615, t15640, t15642, t15643, t15645) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1420(t1226, t4965, t11697, t4953, t3577, t1229, t3242, t13969, t4979, t3506, t4973, t1227);
    (t15594, t15608, t15610, t15615, t15640, t15642, t15643, t15645)
}
