//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 974/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk974<F: Float>(t22840: F, t22874: F, t22903: F, t22962: F, t225: F, t1903: F, t6918: F, t4076: F, t6895: F, t9657: F, t13727: F, t1424: F, t213: F, t22400: F, t22405: F, t22407: F, t22410: F, t561: F, t5715: F, t6896: F, t9639: F, t9650: F, t9666: F, t9691: F, t9694: F) -> (F, F, F, F, F) {
    let t22964 = t22840 + t22874 + t22903 + t22962;
    let t22965 = t22964 * t225;
    let t22970 = t1903 * t6918;
    let t22971 = t4076 * t22970;
    let t22974 = t6895 * t1903;
    let t22975 = t9657 * t22974;
    let t22984 = t9639 + t9650 + F::cast_from(0.65854491829355115987e0_f64) * t213 * t22965 * t561 - F::cast_from(0.19514881078765566038e-2_f64) * t13727 + F::cast_from(0.39512695097613069591e1_f64) * t1424 * t22971 - F::cast_from(0.39512695097613069591e1_f64) * t1424 * t22975 - t9666 + F::cast_from(0.39512695097613069591e1_f64) * t5715 * t6896 - F::cast_from(0.29272321618148349057e-1_f64) * t22400 + F::cast_from(0.29272321618148349057e-1_f64) * t22405 - F::cast_from(0.58544643236296698113e-1_f64) * t22407 + F::cast_from(0.16463622957338778996e-1_f64) * t22410 - t9691 + t9694;
    (t22964, t22971, t22974, t22975, t22984)
}
