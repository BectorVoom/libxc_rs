//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3698/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3698<F: Float>(t17306: F, t17728: F, t489: F, t1261: F, t12879: F, t247: F, t6425: F, t12772: F, t21227: F, t3625: F, t21021: F, t12855: F, t13396: F, t16714: F, t17254: F, t17461: F, t17739: F, t20795: F, t21143: F, t21203: F, t3584: F, t3603: F, t3620: F, t3626: F, t3720: F, t44225: F, t5402: F, t56853: F, t56867: F, t57005: F, t57275: F, t57571: F, t59411: F, t6638: F) -> F {
    let t70014 = t17306 * t489 * t17728;
    let t70032 = t1261 * t247 * t12879 * t6425;
    let t70039 = t3625 * t12772 * t21227;
    let t70044 = t3625 * t12772 * t21021;
    let t70050 = -F::cast_from(0.11433071498151929859e-2_f64) * t70014 * t17739 - F::cast_from(0.2540682555144873302e-2_f64) * t57005 * t44225 * t16714 * t13396 - F::cast_from(0.42874018118069736972e-3_f64) * t12855 * t3720 * t20795 * t3603 * t3584 + F::cast_from(0.23818898954483187207e-3_f64) * t21143 * t3620 - F::cast_from(0.91464571985215438872e-2_f64) * t21203 * t17254 + F::cast_from(0.6351706387862183255e-4_f64) * t70032 + F::cast_from(0.17149607247227894789e-2_f64) * t59411 * t17461 + F::cast_from(0.3811023832717309953e-3_f64) * t56853 - F::cast_from(0.76220476654346199061e-3_f64) * t56867 - F::cast_from(0.3811023832717309953e-3_f64) * t70039 + F::cast_from(0.30488190661738479624e-2_f64) * t57571 * t5402 - F::cast_from(0.3811023832717309953e-3_f64) * t70044 - F::cast_from(0.28582678745379824648e-3_f64) * t3625 * t3626 * t57275 * t6638;
    t70050
}
