//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 722/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk722(t235: f64, t29837: f64, t1652: f64, t321: f64, t234: f64, t833: f64, t503: f64, t325: f64, t6477: f64, t622: f64, t794: f64, t117: f64, t28317: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t29838 = t235 * t29837;
    let t29892 = t1652 * t321;
    let t29927 = t234 * t833;
    let t29933 = t503 * t321;
    let t30080 = t6477 * t325;
    let t30137 = t622 * t794;
    let t30174 = t28317 * t117;
    (t29838, t29892, t29927, t29933, t30080, t30137, t30174)
}
