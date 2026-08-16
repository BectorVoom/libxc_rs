//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 946/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk946(t1730: f64, t3564: f64, t10603: f64, t10607: f64, t10611: f64, t10614: f64, t10616: f64, t10618: f64, t10620: f64, t10622: f64, t10626: f64, t10628: f64, t10631: f64, t10633: f64, t10634: f64, t10657: f64, t256: f64, t267: f64, t7474: f64, t7478: f64) -> (f64, f64) {
    let t10661 = 4.0_f64 / 15.0_f64 * t1730 * t3564;
    let t10662 = t10603 * t256 / 3.0_f64 + t10607 / 3.0_f64 + 0.60777777777777777777e-1_f64 * t10611 - t7474 - t7478 - t10614 + t10616 - t10618 - t10620 - t10622 - t10626 + t10628 + t10631 + t10633 - 2.0_f64 / 45.0_f64 * t10634 - t10657 * t267 / 15.0_f64 + t10661;
    (t10661, t10662)
}
