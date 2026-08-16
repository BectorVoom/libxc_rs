//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 252/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk252(t2652: f64, t2399: f64, t313: f64, t89: f64, t1882: f64, t842: f64, t877: f64, t681: f64, t865: f64, t311: f64, t869: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2793 = 4.0_f64 / 9.0_f64 * t2652;
    let t2816 = 4.0_f64 / 27.0_f64 * t89 * t2399 * t313;
    let t2817 = t1882 * t842;
    let t2819 = t1882 * t877;
    let t2823 = 4.0_f64 / 27.0_f64 * t2652;
    let t2839 = t89 * t681 * t865;
    let t2842 = 1.0_f64 / t869 / t311;
    (t2793, t2816, t2817, t2819, t2823, t2839, t2842)
}
