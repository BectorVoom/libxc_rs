//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2055/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2055(t26024: f64, t3926: f64, t4059: f64, t2482: f64, t25981: f64, t27: f64, t10003: f64, t25997: f64, t9970: f64, t550: f64, t7021: f64, t3946: f64) -> (f64, f64, f64, f64, f64) {
    let t94503 = t26024 * t3926;
    let t94505 = t26024 * t4059;
    let t94508 = t2482 * t25981 * t27;
    let t94509 = t94508 * t10003;
    let t94511 = t25997 * t9970;
    let t94513 = t7021 * t550;
    let t94514 = t94513 * t3946;
    (t94503, t94505, t94509, t94511, t94514)
}
