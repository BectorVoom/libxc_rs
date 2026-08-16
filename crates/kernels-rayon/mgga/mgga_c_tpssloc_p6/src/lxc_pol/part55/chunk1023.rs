//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1023/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1023(t1442: f64, t1774: f64, t2114: f64, t25975: f64, t25977: f64, t25979: f64, t25982: f64, t25987: f64, t25991: f64, t25993: f64, t25996: f64, t25998: f64, t26002: f64, t26005: f64, t27863: f64, t5107: f64, t672: f64, t7264: f64, t7408: f64) -> f64 {
    let t27867 = -t1442 * t7408 - t1774 * t7264 - t2114 * t5107 - 2.0_f64 * t27863 * t672 - t25975 - t25977 - t25979 - t25982 + t25987 - t25991 - t25993 - t25996 - t25998 - t26002 - t26005;
    t27867
}
