//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1033/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1033(t3555: f64, t3754: f64, t1248: f64, t3153: f64, t3566: f64, t1269: f64, t1284: f64, t1209: f64, t1204: f64, t3781: f64, t5462: f64, t5477: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12709 = t3555 * t3754;
    let t12712 = t1248 * t3153;
    let t12717 = t3566 * t3754;
    let t12722 = t1284 * t1269;
    let t12723 = t1209 * t12722;
    let t12744 = t1204 * t3781;
    let t12751 = t1209 * t5462;
    let t12756 = t1209 * t5477;
    (t12709, t12712, t12717, t12723, t12744, t12751, t12756)
}
