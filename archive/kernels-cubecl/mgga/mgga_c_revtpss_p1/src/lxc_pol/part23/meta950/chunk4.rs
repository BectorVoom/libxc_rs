//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3146/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3146<F: Float>(t1042: F, t1260: F, t1261: F, t1264: F, t17412: F, t17693: F, t17763: F, t20946: F, t20947: F, t21093: F, t21153: F, t247: F, t24858: F, t3617: F, t3647: F, t372: F, t5056: F, t5381: F, t5384: F, t5386: F, t5391: F, t6622: F, t6679: F, t6683: F, t69637: F, t71440: F, t81226: F) -> F {
    let t82639 = -F::cast_from(0.85748036236139473944e-3_f64) * t5384 * t1042 * t21093 * t5056 + F::cast_from(0.22866142996303859718e-2_f64) * t17412 * t6679 + F::cast_from(0.22866142996303859718e-2_f64) * t5391 * t21153 + F::cast_from(0.45732285992607719436e-2_f64) * t17412 * t6683 - F::cast_from(0.42874018118069736972e-3_f64) * t17763 * t6679 - F::cast_from(0.42874018118069736972e-3_f64) * t5381 * t21153 - F::cast_from(0.14291339372689912324e-3_f64) * t3647 * t24858 - F::cast_from(0.14291339372689912324e-3_f64) * t1261 * t247 * t1264 * t81226 + F::cast_from(0.12862205435420921092e-2_f64) * t69637 * t1260 * t5386 + F::cast_from(0.7145669686344956162e-3_f64) * t17693 * t372 * t3617 * t6622 * t20946 - F::cast_from(0.7622047665434619906e-2_f64) * t71440 * t20947;
    t82639
}
