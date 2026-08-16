//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1465/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1465(t103103: f64, t105131: f64, t105144: f64, t105146: f64, t105147: f64, t105150: f64, t109982: f64, t110002: f64, t1398: f64, t1852: f64, t1858: f64, t2170: f64, t2174: f64, t22431: f64, t22453: f64, t29866: f64, t29884: f64, t3: f64, t580: f64, t6471: f64, t6483: f64, t8111: f64, t8119: f64) -> f64 {
    let tv4rho3sigma11 = t109982 * t3 * t580 + t110002 * t1398 + 3.0_f64 * t1852 * t29884 + 3.0_f64 * t1858 * t29866 + t2170 * t22453 + t2174 * t22431 + 3.0_f64 * t6471 * t8119 + 3.0_f64 * t6483 * t8111 + 3.0_f64 * t103103 + 6.0_f64 * t105131 + 6.0_f64 * t105144 + 3.0_f64 * t105146 + 3.0_f64 * t105147 + 3.0_f64 * t105150;
    tv4rho3sigma11
}
