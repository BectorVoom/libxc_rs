//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3550/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3550<F: Float>(t1063: F, t11986: F, t247: F, t6096: F, t1045: F, t15785: F, t19572: F, t3115: F, t3117: F, t3120: F, t4892: F, t4894: F, t55293: F, t55320: F, t55325: F, t55328: F, t55361: F, t55367: F, t66565: F, t67545: F, t67551: F, t67560: F, t67568: F, t67571: F) -> F {
    let t67575 = t1063 * t247 * t11986 * t6096;
    let t67578 = F::cast_from(0.19055119163586549765e-3_f64) * t55293 - F::cast_from(0.42874018118069736972e-3_f64) * t3115 * t3117 * t67545 * t1045 - F::cast_from(0.42874018118069736972e-3_f64) * t67551 * t3120 + F::cast_from(0.42874018118069736972e-3_f64) * t4892 * t3117 * t19572 * t15785 - F::cast_from(0.28582678745379824648e-3_f64) * t55320 + F::cast_from(0.11433071498151929859e-2_f64) * t67560 + F::cast_from(0.85748036236139473944e-3_f64) * t4892 * t3117 * t66565 * t4894 - F::cast_from(0.17149607247227894789e-2_f64) * t55325 + F::cast_from(0.28582678745379824648e-3_f64) * t55328 + F::cast_from(0.28582678745379824648e-3_f64) * t67568 + F::cast_from(0.11433071498151929859e-2_f64) * t55361 + F::cast_from(0.20325460441158986416e-2_f64) * t67571 + F::cast_from(0.6351706387862183255e-4_f64) * t67575 + F::cast_from(0.11433071498151929859e-2_f64) * t55367;
    t67578
}
