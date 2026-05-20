//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3710/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3710<F: Float>(t13058: F, t20786: F, t11262: F, t3711: F, t6618: F, t1261: F, t21110: F, t3172: F, t1042: F, t12784: F, t17232: F, t20792: F, t21219: F, t3647: F, t3674: F, t5268: F, t5391: F, t57063: F, t57070: F, t65433: F, t70263: F, t70265: F, t70267: F, t70270: F, t70273: F) -> F {
    let t70275 = t13058 * t20786;
    let t70278 = t3711 * t11262 * t6618;
    let t70281 = t1261 * t3172 * t21110;
    let t70289 = F::cast_from(0.47637797908966374414e-3_f64) * t3647 * t20792 - F::cast_from(0.57165357490759649296e-3_f64) * t1261 * t1042 * t5268 * t65433 + F::cast_from(0.47637797908966374413e-4_f64) * t70263 + F::cast_from(0.30488190661738479624e-2_f64) * t70265 - F::cast_from(0.45732285992607719436e-2_f64) * t70267 * t3674 - F::cast_from(0.76220476654346199061e-3_f64) * t70270 + F::cast_from(0.31758531939310916276e-3_f64) * t70273 - F::cast_from(0.28582678745379824648e-3_f64) * t70275 - F::cast_from(0.6351706387862183255e-4_f64) * t70278 - F::cast_from(0.8468941850482911007e-3_f64) * t70281 + F::cast_from(0.60976381323476959248e-2_f64) * t5391 * t17232 - F::cast_from(0.28582678745379824648e-3_f64) * t12784 * t21219 + F::cast_from(0.57165357490759649296e-3_f64) * t57063 + F::cast_from(0.11433071498151929859e-2_f64) * t57070;
    t70289
}
