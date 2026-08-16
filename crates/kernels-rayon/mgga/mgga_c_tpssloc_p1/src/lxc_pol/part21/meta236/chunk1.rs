//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1402/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1402(t340: f64, t5842: f64, t343: f64, t974: f64, t2969: f64, t2986: f64, t4507: f64, t4529: f64, t5818: f64, t5821: f64, t5825: f64, t5829: f64, t5839: f64, t973: f64) -> (f64, f64, f64) {
    let t5843 = t340 * t5842;
    let t5844 = t5843 * t343;
    let t5845 = t974 * t5844;
    let t5848 = -t2969 + 0.18518518518518518518e-3_f64 * t4507 - 0.55555555555555555554e-3_f64 * t4529 + 0.37037037037037037036e-3_f64 * t973 * t5818 - 0.55555555555555555554e-3_f64 * t2986 * t5821 - 0.55555555555555555554e-3_f64 * t973 * t5825 + 0.27777777777777777777e-3_f64 * t973 * t5829 - 0.83333333333333333332e-3_f64 * t973 * t5839 - 0.83333333333333333332e-3_f64 * t973 * t5845;
    (t5844, t5845, t5848)
}
