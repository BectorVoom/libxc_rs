//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1368/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1368<F: Float>(t3628: F, t4186: F, t5351: F, t3626: F, t12910: F, t17283: F, t17375: F, t17448: F, t17605: F, t1791: F, t21001: F, t21004: F, t21008: F, t21014: F, t21017: F, t3625: F, t5320: F, t5323: F, t5335: F, t5343: F, t5402: F, t5407: F) -> F {
    let t21020 = t3628 * t4186;
    let t21021 = t5351 * t21020;
    let t21022 = t3626 * t21021;
    let t21027 = F::cast_from(0.22866142996303859718e-2_f64) * t17283 * t1791 + F::cast_from(0.22866142996303859718e-2_f64) * t5323 * t5320 - F::cast_from(0.28582678745379824648e-3_f64) * t17448 * t5407 + t17375 + F::cast_from(0.10162730220579493208e-2_f64) * t21001 + F::cast_from(0.85748036236139473944e-3_f64) * t12910 * t21004 + F::cast_from(0.23818898954483187207e-3_f64) * t3625 * t21008 + F::cast_from(0.15244095330869239812e-2_f64) * t17605 * t5407 - F::cast_from(0.45732285992607719436e-2_f64) * t21014 * t5343 + F::cast_from(0.22866142996303859718e-2_f64) * t21017 * t5335 - F::cast_from(0.28582678745379824648e-3_f64) * t3625 * t21022 - F::cast_from(0.28582678745379824648e-3_f64) * t17448 * t5402;
    t21027
}
