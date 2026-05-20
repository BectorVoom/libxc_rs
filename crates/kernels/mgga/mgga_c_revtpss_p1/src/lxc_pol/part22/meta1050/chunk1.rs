//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3696/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3696<F: Float>(t17361: F, t5293: F, t1042: F, t1252: F, t17222: F, t17505: F, t17547: F, t17796: F, t1797: F, t20809: F, t3363: F, t3368: F, t3617: F, t3711: F, t3714: F, t5274: F, t5287: F, t5384: F, t59371: F, t6573: F, t69947: F, t69958: F, t69961: F, t69964: F, t69966: F, t69968: F) -> F {
    let t69971 = t5293 * t17361;
    let t69982 = -F::cast_from(0.20325460441158986416e-2_f64) * t69947 + F::cast_from(0.28582678745379824648e-3_f64) * t3711 * t1042 * t20809 * t3368 - F::cast_from(0.22866142996303859718e-2_f64) * t59371 * t1797 - F::cast_from(0.45732285992607719436e-2_f64) * t17547 * t5287 + F::cast_from(0.42874018118069736972e-3_f64) * t69958 * t1252 - F::cast_from(0.33875767401931644026e-2_f64) * t69961 + F::cast_from(0.47637797908966374413e-4_f64) * t69964 + F::cast_from(0.3811023832717309953e-3_f64) * t69966 - F::cast_from(0.30488190661738479624e-2_f64) * t69968 * t3714 + F::cast_from(0.5081365110289746604e-3_f64) * t69971 + F::cast_from(0.47637797908966374413e-3_f64) * t5384 * t1042 * t3617 * t6573 * t3363 + F::cast_from(0.2540682555144873302e-2_f64) * t17505 * t17796 + F::cast_from(0.42874018118069736972e-3_f64) * t5274 * t17222;
    t69982
}
