//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 432/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk432(t1134: f64, t1138: f64, t418: f64, t971: f64, t977: f64, t431: f64, t1038: f64, t416: f64, t4189: f64, t1028: f64, t385: f64, t381: f64) -> (f64, f64, f64, f64, f64) {
    let t4352 = t1134 * t1138;
    let t4359 = t977 * t971 * t418;
    let t4361 = 0.35089341735807877242e1_f64 * t431 * t4359;
    let t4363 = t1038 * t416 * t4189;
    let t4365 = 0.51947577317044391277e2_f64 * t431 * t4363;
    let t4366 = t385 * t1028;
    let t4372 = t381 * t1028;
    (t4352, t4361, t4365, t4366, t4372)
}
