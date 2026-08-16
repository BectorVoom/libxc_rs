//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2748/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2748(t57947: f64, t12971: f64, t2522: f64, t39397: f64, t39400: f64, t39408: f64, t39411: f64, t40708: f64, t4310: f64, t4314: f64, t4315: f64, t57932: f64, t57936: f64, t57939: f64, t57943: f64, t57946: f64, t776: f64) -> (f64, f64) {
    let t57948 = 8.0_f64 * t57947;
    let t57955 = 6.0_f64 * t12971 * t2522 * t4310 + 12.0_f64 * t12971 * t4314 * t4315 + 6.0_f64 * t2522 * t57932 * t776 - t39397 - t39400 + t39408 + t39411 + t40708 + t57936 + t57939 + t57943 + t57946 + t57948;
    (t57948, t57955)
}
