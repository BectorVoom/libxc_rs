//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3717/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3717<F: Float>(t3666: F, t6594: F, t17283: F, t5362: F, t1238: F, t12832: F, t17280: F, t17405: F, t17672: F, t1791: F, t20851: F, t21042: F, t21177: F, t3625: F, t3626: F, t3663: F, t5320: F, t5323: F, t5373: F, t57173: F, t57176: F, t57178: F, t59025: F, t6429: F) -> F {
    let t70469 = t3666 * t6594;
    let t70476 = t17283 * t5362;
    let t70480 = -F::cast_from(0.14291339372689912324e-3_f64) * t3625 * t3626 * t6429 * t17672 + F::cast_from(0.11433071498151929859e-2_f64) * t57173 - F::cast_from(0.3811023832717309953e-3_f64) * t57176 + F::cast_from(0.28582678745379824648e-3_f64) * t57178 - F::cast_from(0.42874018118069736972e-3_f64) * t12832 * t21042 + F::cast_from(0.22866142996303859718e-2_f64) * t59025 * t1791 + F::cast_from(0.45732285992607719436e-2_f64) * t17283 * t5320 + F::cast_from(0.22866142996303859718e-2_f64) * t5323 * t17280 - F::cast_from(0.14481890564325777821e-1_f64) * t70469 * t1238 - F::cast_from(0.72409452821628889107e-2_f64) * t21177 * t3663 - F::cast_from(0.21437009059034868486e-3_f64) * t20851 * t3663 + F::cast_from(0.30488190661738479624e-2_f64) * t70476 + t5373 * t17405 / F::cast_from(54.0_f64);
    t70480
}
