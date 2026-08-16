//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 519/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk519(t676: f64, t886: f64, t123: f64, t2465: f64, t215: f64, t685: f64) -> (f64, f64, f64, f64) {
    let t2466 = t676 * t886;
    let t2467 = t123 * t2466;
    let t2468 = t2465 * t2467;
    let t2470 = t685 * t215;
    (t2466, t2467, t2468, t2470)
}
