//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 801/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk801(t1018: f64, t4992: f64, t86: f64, t1022: f64, t4621: f64, t1021: f64, t1808: f64, sigma0: f64) -> (f64, f64, f64, f64, f64) {
    let t4994 = t86 * t4992 * t1018;
    let t4995 = t1022 * t4621;
    let t4996 = t1021 * t4995;
    let t4997 = t4994 * t4996;
    let t4999 = t1808 * sigma0;
    (t4994, t4995, t4996, t4997, t4999)
}
