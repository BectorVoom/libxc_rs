//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1249/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1249(t3634: f64, t828: f64, t3624: f64, t3746: f64, t3618: f64, t1209: f64, t3781: f64, t5330: f64, t1284: f64, t3555: f64, t1121: f64, t3603: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12772 = t828 * t3634;
    let t12784 = t3746 * t3624;
    let t12787 = t828 * t3618;
    let t12808 = t1209 * t3781;
    let t12809 = t12808 * t5330;
    let t12831 = t3555 * t1284;
    let t12832 = t12831 * t3624;
    let t12839 = t3603 * t1121;
    (t12772, t12784, t12787, t12809, t12832, t12839)
}
