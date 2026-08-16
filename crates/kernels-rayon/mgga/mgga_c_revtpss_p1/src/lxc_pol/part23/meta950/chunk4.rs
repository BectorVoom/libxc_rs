//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3146/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3146(t1042: f64, t1260: f64, t1261: f64, t1264: f64, t17412: f64, t17693: f64, t17763: f64, t20946: f64, t20947: f64, t21093: f64, t21153: f64, t247: f64, t24858: f64, t3617: f64, t3647: f64, t372: f64, t5056: f64, t5381: f64, t5384: f64, t5386: f64, t5391: f64, t6622: f64, t6679: f64, t6683: f64, t69637: f64, t71440: f64, t81226: f64) -> f64 {
    let t82639 = -0.85748036236139473944e-3_f64 * t5384 * t1042 * t21093 * t5056 + 0.22866142996303859718e-2_f64 * t17412 * t6679 + 0.22866142996303859718e-2_f64 * t5391 * t21153 + 0.45732285992607719436e-2_f64 * t17412 * t6683 - 0.42874018118069736972e-3_f64 * t17763 * t6679 - 0.42874018118069736972e-3_f64 * t5381 * t21153 - 0.14291339372689912324e-3_f64 * t3647 * t24858 - 0.14291339372689912324e-3_f64 * t1261 * t247 * t1264 * t81226 + 0.12862205435420921092e-2_f64 * t69637 * t1260 * t5386 + 0.7145669686344956162e-3_f64 * t17693 * t372 * t3617 * t6622 * t20946 - 0.7622047665434619906e-2_f64 * t71440 * t20947;
    t82639
}
