//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1361/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1361(t1219: f64, t6419: f64, t1265: f64, t12957: f64, t13108: f64, t1838: f64, t18483: f64, t18490: f64, t18496: f64, t18499: f64, t18967: f64, t18968: f64, t19521: f64, t19535: f64, t20182: f64, t20187: f64, t20190: f64, t20196: f64, t20202: f64, t3366: f64, t3384: f64, t4516: f64, t520: f64, t5739: f64, t5740: f64, t5745: f64, t5918: f64, t60649: f64, t60653: f64, t60811: f64, t62508: f64, t6424: f64, t65691: f64, t65696: f64, t65711: f64, t65715: f64, t65719: f64, t65722: f64, t65867: f64, t65871: f64) -> f64 {
    let t66970 = t1219 * t6419;
    let t66998 = -12.0_f64 * t5739 * t18490 * t20182 * t1265 - 2.0_f64 * t18496 * t18967 * t65711 - 4.0_f64 * t18496 * t62508 * t19521 - 4.0_f64 * t18496 * t18967 * t65691 + 2.0_f64 * t5739 * t5740 * t1838 * t13108 - 4.0_f64 * t65871 * t18968 + 4.0_f64 * t18496 * t20190 * t65696 + 6.0_f64 * t60653 * t18967 * t65722 + 4.0_f64 * t5739 * t5740 * t5918 * t4516 - 2.0_f64 * t18496 * t18967 * t65715 - 4.0_f64 * t18496 * t66970 * t18499 - 4.0_f64 * t60649 * t20187 + 2.0_f64 * t65719 * t20202 - 4.0_f64 * t18496 * t62508 * t19535 - 4.0_f64 * t18496 * t18967 * t65867 + 24.0_f64 * t5739 * t60811 * t6424 * t3366 + t5739 * t5745 * t1838 * t12957 * t520 + 2.0_f64 * t18483 * t20196 + 2.0_f64 * t5739 * t5740 * t6419 * t3384;
    t66998
}
