//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1111/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1111(t1820: f64, t1885: f64, t31352: f64, t3454: f64, t40865: f64, t40867: f64, t47556: f64, t7062: f64, t7063: f64, t40079: f64, t954: f64, t7115: f64, t7759: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t47760 = 16.0_f64 / 5.0_f64 * t1820 * t1885 * t31352 * t3454;
    let t47761 = 16.0_f64 / 15.0_f64 * t40865;
    let t47762 = 32.0_f64 / 15.0_f64 * t40867;
    let t47765 = 32.0_f64 / 15.0_f64 * t7062 * t7063 * t47556;
    let t47766 = t40079 * t954;
    let t47769 = 16.0_f64 / 9.0_f64 * t7115 * t7759 * t47766;
    (t47760, t47761, t47762, t47765, t47766, t47769)
}
