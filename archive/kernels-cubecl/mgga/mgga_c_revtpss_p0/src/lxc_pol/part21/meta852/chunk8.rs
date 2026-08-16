//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3210/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3210<F: Float>(t480: F, t59241: F, t12287: F, t1250: F, t12705: F, t12712: F, t12784: F, t12832: F, t12938: F, t13022: F, t13046: F, t17351: F, t17353: F, t17354: F, t17391: F, t17505: F, t17638: F, t17669: F, t17693: F, t247: F, t3629: F, t3674: F, t3719: F, t3720: F, t44500: F, t44949: F, t44965: F, t44972: F, t44980: F, t5373: F, t5384: F, t56530: F, t56981: F, t59096: F, t59220: F, t59233: F, t59239: F) -> F {
    let t59242 = t59241 * t480;
    let t59267 = F::cast_from(0.95275595817932748827e-3_f64) * t59220 - F::cast_from(0.12862205435420921092e-2_f64) * t12832 * t17391 + F::cast_from(0.57165357490759649295e-3_f64) * t44949 + F::cast_from(0.3811023832717309953e-2_f64) * t17505 * t12938 - F::cast_from(0.85748036236139473944e-3_f64) * t12784 * t17669 - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t5373 * t13022 + F::cast_from(0.95275595817932748826e-4_f64) * t59233 + F::cast_from(0.12862205435420921092e-2_f64) * t5384 * t247 * t3719 * t56530 + F::cast_from(0.22866142996303859718e-2_f64) * t59239 + F::cast_from(0.12862205435420921092e-2_f64) * t59242 * t3674 - F::cast_from(0.12862205435420921092e-2_f64) * t44500 * t3720 * t59096 * t13046 + F::cast_from(0.85748036236139473944e-3_f64) * t17351 * t56981 * t17354 + F::cast_from(0.42874018118069736972e-3_f64) * t17351 * t17353 * t12705 * t3629 + F::cast_from(0.42874018118069736972e-3_f64) * t17351 * t17353 * t12712 * t17638 - t44965 / F::cast_from(144.0_f64) - F::cast_from(7.0_f64) / F::cast_from(1944.0_f64) * t44972 - t44980 / F::cast_from(324.0_f64) - F::cast_from(0.85748036236139473944e-3_f64) * t17693 * t17353 * t1250 * t12287;
    t59267
}
