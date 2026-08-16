//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3210/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3210(t480: f64, t59241: f64, t12287: f64, t1250: f64, t12705: f64, t12712: f64, t12784: f64, t12832: f64, t12938: f64, t13022: f64, t13046: f64, t17351: f64, t17353: f64, t17354: f64, t17391: f64, t17505: f64, t17638: f64, t17669: f64, t17693: f64, t247: f64, t3629: f64, t3674: f64, t3719: f64, t3720: f64, t44500: f64, t44949: f64, t44965: f64, t44972: f64, t44980: f64, t5373: f64, t5384: f64, t56530: f64, t56981: f64, t59096: f64, t59220: f64, t59233: f64, t59239: f64) -> f64 {
    let t59242 = t59241 * t480;
    let t59267 = 0.95275595817932748827e-3_f64 * t59220 - 0.12862205435420921092e-2_f64 * t12832 * t17391 + 0.57165357490759649295e-3_f64 * t44949 + 0.3811023832717309953e-2_f64 * t17505 * t12938 - 0.85748036236139473944e-3_f64 * t12784 * t17669 - 2.0_f64 / 27.0_f64 * t5373 * t13022 + 0.95275595817932748826e-4_f64 * t59233 + 0.12862205435420921092e-2_f64 * t5384 * t247 * t3719 * t56530 + 0.22866142996303859718e-2_f64 * t59239 + 0.12862205435420921092e-2_f64 * t59242 * t3674 - 0.12862205435420921092e-2_f64 * t44500 * t3720 * t59096 * t13046 + 0.85748036236139473944e-3_f64 * t17351 * t56981 * t17354 + 0.42874018118069736972e-3_f64 * t17351 * t17353 * t12705 * t3629 + 0.42874018118069736972e-3_f64 * t17351 * t17353 * t12712 * t17638 - t44965 / 144.0_f64 - 7.0_f64 / 1944.0_f64 * t44972 - t44980 / 324.0_f64 - 0.85748036236139473944e-3_f64 * t17693 * t17353 * t1250 * t12287;
    t59267
}
