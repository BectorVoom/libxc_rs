//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 726/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk726(t209: f64, t5666: f64, t1540: f64, t325: f64, t107: f64, t1539: f64, t837: f64, t874: f64, t235: f64, t1652: f64, t321: f64, t234: f64, t833: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t27724 = t5666 * t209;
    let t28295 = t1540 * t325;
    let t28317 = t1539 * t107;
    let t29837 = t837 * t874;
    let t29838 = t235 * t29837;
    let t29892 = t1652 * t321;
    let t29927 = t234 * t833;
    (t27724, t28295, t28317, t29837, t29838, t29892, t29927)
}
