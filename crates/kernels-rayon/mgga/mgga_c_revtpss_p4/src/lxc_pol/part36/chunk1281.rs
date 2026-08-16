//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1281/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1281(t27873: f64, t98041: f64, t22453: f64, t94901: f64, t108368: f64, t25895: f64, t108187: f64, t25878: f64, t30081: f64, t689: f64, t94768: f64, t94763: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t108440 = t98041 * t27873;
    let t108455 = t94901 * t22453;
    let t108464 = t25895 * t108368;
    let t108474 = t25878 * t108187;
    let t108493 = t30081 * t689;
    let t108494 = t94768 * t108493;
    let t108496 = t94763 * t108493;
    (t108440, t108455, t108464, t108474, t108494, t108496)
}
