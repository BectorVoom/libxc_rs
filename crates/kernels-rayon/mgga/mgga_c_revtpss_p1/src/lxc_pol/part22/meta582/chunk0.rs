//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2442/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2442(t18616: f64, t827: f64, t828: f64, t221: f64, t2485: f64, t6017: f64, t2484: f64, t125: f64, t5962: f64, t2747: f64, t837: f64, t2723: f64, t4423: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18618 = t827 * t828 * t18616;
    let t18622 = t2485 * t221 * t6017;
    let t18623 = t2484 * t18622;
    let t18627 = t125 * t5962;
    let t18629 = t2747 * t18627 * t837;
    let t18632 = t2723 * t4423;
    (t18618, t18622, t18623, t18627, t18629, t18632)
}
