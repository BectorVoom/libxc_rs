//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 993/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk993(t10504: f64, t10509: f64, t10512: f64, t10596: f64, t10597: f64, t10599: f64, t10600: f64, t10614: f64, t10616: f64, t10618: f64, t10620: f64, t10622: f64, t10626: f64, t10628: f64, t10631: f64, t7474: f64, t7478: f64) -> f64 {
    let t11206 = t10504 - t10509 + t10512 - t10596 - t10597 - t10599 - t10600 - t7474 - t7478 - t10614 + t10616 - t10618 - t10620 - t10622 - t10626 + t10628 + t10631;
    t11206
}
