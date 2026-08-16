//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1157/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1157(t1285: f64, t26866: f64, t3717: f64, t3707: f64, t7617: f64, t2134: f64, t3682: f64, t1234: f64, t7623: f64, t1252: f64, t1266: f64, t26849: f64, t26852: f64, t26855: f64, t26863: f64, t3591: f64, t3613: f64, t3620: f64, t3631: f64, t3640: f64, t3644: f64, t3714: f64, t3723: f64, t7618: f64, t7624: f64) -> (f64, f64, f64, f64, f64) {
    let t26867 = t1285 * t26866;
    let t26870 = t3717 * t26866;
    let t26873 = t3707 * t7617;
    let t26877 = t2134 * t3682 / 432.0_f64;
    let t26880 = t1234 * t7623;
    let t26883 = -0.42874018118069736972e-3_f64 * t26849 * t3613 - 0.57165357490759649296e-3_f64 * t26852 * t1266 - 0.3811023832717309953e-3_f64 * t26855 - 0.28582678745379824648e-3_f64 * t7624 * t3640 - 0.57165357490759649296e-3_f64 * t7624 * t3644 + 0.47637797908966374413e-3_f64 * t7624 * t3620 + 0.57165357490759649296e-3_f64 * t26863 - 0.57165357490759649296e-3_f64 * t26867 * t3631 - 0.85748036236139473944e-3_f64 * t26870 * t3723 + 0.85748036236139473944e-3_f64 * t26873 * t1252 - t26877 + 0.42874018118069736972e-3_f64 * t7618 * t3591 + 0.57165357490759649296e-3_f64 * t26880 * t3714;
    (t26867, t26870, t26873, t26880, t26883)
}
